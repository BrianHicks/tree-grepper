use clap::Clap;
use ignore::{types, WalkBuilder, WalkState};
use std::fs;
use std::path::PathBuf;
use std::process;
use std::str::Utf8Error;
use tree_sitter::{self, Query, QueryCursor};

#[derive(Clap, Debug)]
#[clap(version = "1.0")]
struct Opts {
    /// Pattern to search for.
    pattern: String,

    /// Paths to look for files. Can be files, directories, or a mix of both.
    #[clap(default_value = ".", parse(from_os_str))]
    paths: Vec<PathBuf>,

    /// How deeply to recurse (default: no limit)
    #[clap(short('d'), long)]
    max_depth: Option<usize>,

    /// Follow symlinks
    #[clap(short('f'), long("follow"))]
    follow_links: bool,

    /// How many threads to use when loading files (default: choose automatically based on heuristics from ripgrep)
    #[clap(long, default_value = "0")]
    threads: usize,
    // TODO: add more options from https://docs.rs/ignore/0.4.16/ignore/struct.WalkBuilder.html
}

fn main() {
    let opts: Opts = Opts::parse();

    // safety checks: can we get a parser?
    if let Err(err) = parser(language_elm()) {
        eprintln!(
            "Couldn't get the parser because of an internal error: {:?}",
            err
        );
        process::exit(1);
    }

    // safety check: is the query acceptable?
    // TODO: this error type has rich enough text to make a really nice error
    // message, but this implementation ends up pretty crappy. Make it better!
    if let Err(err) = Query::new(language_elm(), &opts.pattern) {
        eprintln!("Invalid pattern: {:?}", err);
        process::exit(1);
    }

    // I *think* we should be OK to assume that there's at least one path in
    // this `opts.paths`, since there will be a default set above. This code
    // is a little incautious as a result, and a future refactor could break
    // it! Is there a better way? (e.g. making it impossible by construction
    // like `(a, Vec<a>)`?)
    let mut builder = WalkBuilder::new(opts.paths[0].clone());

    // argh! how do I iterate starting at index 1? Is this the right way?
    let mut idx = 1;
    while let Some(path) = opts.paths.get(idx) {
        builder.add(path);
        idx += 1;
    }

    // TODO: move type definitions to another function
    let mut types_builder = types::TypesBuilder::new();
    if let Err(err) = types_builder.add("elm", "*.elm") {
        eprintln!("Couldn't add Elm type: {:?}", err);
        process::exit(1);
    }
    types_builder.select("elm");

    let types = match types_builder.build() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Couldn't select file types: {:?}", err);
            process::exit(1);
        }
    };

    builder
        .max_depth(opts.max_depth)
        .follow_links(opts.follow_links)
        .threads(opts.threads)
        .types(types)
        .build_parallel()
        .run(|| {
            let mut parser = match parser(language_elm()) {
                Ok(p) => p,
                Err(_) => return Box::new(|_| WalkState::Quit),
            };

            let query = match Query::new(language_elm(), &opts.pattern) {
                Ok(q) => q,
                Err(_) => return Box::new(|_| WalkState::Quit),
            };

            Box::new(move |dir_entry_result| match dir_entry_result {
                Err(err) => {
                    eprintln!("Error reading path: {:}", err);
                    WalkState::Quit
                }
                Ok(dir_entry) => {
                    if dir_entry.path().is_dir() {
                        return WalkState::Continue;
                    }

                    let source = match fs::read_to_string(dir_entry.path()) {
                        Ok(s) => s,
                        Err(err) => {
                            eprintln!("Couldn't read source of {:?}: {:}", dir_entry.path(), err);
                            return WalkState::Quit;
                        }
                    };

                    let tree = match parser.parse(&source, None) {
                        Some(t) => t,
                        None => {
                            eprintln!("Couldn't parse source of {:?}", dir_entry.path());
                            return WalkState::Quit;
                        }
                    };

                    let matches = QueryCursor::new()
                        // TODO: what's this third argument? It's called `text_callback` in the docs?
                        .matches(&query, tree.root_node(), |_| [])
                        .flat_map(|query_match| query_match.captures)
                        .map(|capture| {
                            capture
                                .node
                                .utf8_text(source.as_ref())
                                .map(|capture_source| Match {
                                    position: capture.node.start_position(),
                                    source: String::from(capture_source),
                                })
                        })
                        .collect::<Result<Vec<Match>, Utf8Error>>();

                    match matches {
                        Ok(matches) => {
                            for match_ in matches {
                                println!(
                                    "{:}:{}:{}:{}",
                                    dir_entry.path().display(),
                                    match_.position.row,
                                    match_.position.column,
                                    match_.source
                                )
                            }
                        }
                        Err(err) => {
                            eprintln!(
                                "Couldn't stringify matches in {:?}: {:?}",
                                dir_entry.path(),
                                err
                            );
                            return WalkState::Quit;
                        }
                    }

                    WalkState::Continue
                }
            })
        });
}

#[derive(Debug)]
struct Match {
    position: tree_sitter::Point,
    source: String,
}

// tree-sitter setup

extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

fn language_elm() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}

fn parser(
    language: tree_sitter::Language,
) -> Result<tree_sitter::Parser, tree_sitter::LanguageError> {
    let mut parser = tree_sitter::Parser::new();

    parser.set_language(language)?;

    Ok(parser)
}
