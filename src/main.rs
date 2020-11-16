use clap::Clap;
use crossbeam::channel;
use ignore::{self, types, ParallelVisitor, ParallelVisitorBuilder, WalkBuilder, WalkState};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use serde_json;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
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

    /// What format should we output matches in? Possible: lines or json.
    #[clap(long, default_value = "lines")]
    format: Format,

    /// How deeply to recurse (default: no limit)
    #[clap(short('d'), long)]
    max_depth: Option<usize>,

    /// Follow symlinks
    #[clap(short('f'), long("follow"))]
    follow_links: bool,

    /// Ignore files above this limit
    #[clap(short('s'), long)]
    max_filesize: Option<u64>,

    /// How many threads to use when loading files (default: choose automatically based on heuristics from ripgrep)
    #[clap(long, default_value = "0")]
    threads: usize,

    /// Ignore a specific file/glob (can be specified multiple times)
    #[clap(short, long)]
    ignore: Vec<PathBuf>,

    /// Don't filter files in the usual ways
    #[clap(long)]
    no_standard_filters: bool,

    /// Don't read hidden files
    #[clap(long)]
    no_hidden: bool,

    /// Don't read from parent directories
    #[clap(long)]
    no_parents: bool,

    /// Don't use .ignore files
    #[clap(long)]
    no_dotignore: bool,

    /// Don't use the global .gitignore file
    #[clap(long)]
    no_global_gitignore: bool,

    /// Don't use repo-local .gitignore files
    #[clap(long)]
    no_gitignore: bool,

    /// Don't use repo-local .git/info/exclude
    #[clap(long)]
    no_git_exclude: bool,

    /// Ignore files using global git ignore rules even outside a repository
    #[clap(long)]
    no_require_git: bool,

    /// Process ignore files case-insensitively
    #[clap(long)]
    ignore_case_insensitive: bool,

    /// Don't cross filesystem boundaries when walking directories
    #[clap(long)]
    same_file_system: bool,

    // Skip reading directories that seem like they might be written to by
    // stdout. Uncommon, but use this if you're writing to a file and the tool
    // seems to be getting into infinite loops.
    #[clap(long)]
    skip_stdout: bool,
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
    let query = match get_query(&opts.pattern) {
        Ok(q) => q,
        Err(err) => {
            eprintln!("Invalid pattern: {:?}", err);
            process::exit(1);
        }
    };

    // I *think* we should be OK to assume that there's at least one path in
    // this `opts.paths`, since there will be a default set above. This code
    // is a little incautious as a result, and a future refactor could break
    // it! Is there a better way? (e.g. making it impossible by construction
    // like `(a, Vec<a>)`?)
    let mut paths = opts.paths.iter();
    let mut builder = match paths.next() {
        Some(path) => WalkBuilder::new(path),
        None => {
            eprintln!("There were no paths in the options. This is an internal error, please report it. If you see this in the wild, get around this by specifying paths to search explicitly.");
            process::exit(1);
        }
    };
    for path in paths {
        builder.add(path);
    }

    for ignore in &opts.ignore {
        if let Some(err) = builder.add_ignore(&ignore) {
            eprintln!("Problem adding ignore for {:?}: {}", ignore, err);
            process::exit(1);
        }
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

    let mut gatherer = Gatherer::new(&query);

    builder
        .max_depth(opts.max_depth)
        .follow_links(opts.follow_links)
        .max_filesize(opts.max_filesize)
        .threads(opts.threads)
        .types(types)
        .standard_filters(!opts.no_standard_filters)
        .hidden(!opts.no_hidden)
        .parents(!opts.no_parents)
        .ignore(!opts.no_dotignore)
        .git_global(!opts.no_global_gitignore)
        .git_ignore(!opts.no_gitignore)
        .git_exclude(!opts.no_git_exclude)
        .require_git(!opts.no_require_git)
        .ignore_case_insensitive(opts.ignore_case_insensitive)
        .same_file_system(opts.same_file_system)
        .skip_stdout(opts.skip_stdout)
        .build_parallel()
        .visit(&mut gatherer);

    let formatter = Formatter::new(opts.format, gatherer);
    formatter.format();
}

// matches

#[derive(Debug)]
struct Point(tree_sitter::Point);

#[derive(Debug, Serialize)]
struct Match {
    path: PathBuf,
    position: Point,
    source: String,
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut match_ = serializer.serialize_struct("Point", 2)?;
        match_.serialize_field("row", &self.0.row)?;
        match_.serialize_field("column", &self.0.column)?;
        match_.end()
    }
}

// visiting nodes

struct Gatherer<'a> {
    query: &'a tree_sitter::Query,
    sender: channel::Sender<Match>,
    receiver: channel::Receiver<Match>,
}

impl<'a> Gatherer<'a> {
    fn new(query: &'a tree_sitter::Query) -> Gatherer<'a> {
        let (sender, receiver) = channel::unbounded();
        Gatherer {
            query: query,
            sender: sender,
            receiver: receiver,
        }
    }
}

impl<'a> ParallelVisitorBuilder<'a> for Gatherer<'a> {
    fn build(&mut self) -> Box<(dyn ParallelVisitor + 'a)> {
        let visitor = Visitor::new(self.sender.clone(), self.query);

        Box::new(visitor)
    }
}

struct Visitor<'a> {
    parser: tree_sitter::Parser,
    query: &'a tree_sitter::Query,
    sender: channel::Sender<Match>,
}

impl<'a> Visitor<'a> {
    fn new(sender: channel::Sender<Match>, query: &tree_sitter::Query) -> Visitor {
        let our_parser = match parser(language_elm()) {
            Ok(p) => p,
            Err(err) => {
                eprintln!(
                    "Couldn't get the parser because of an internal error: {:?}",
                    err
                );
                process::exit(1);
            }
        };

        Visitor {
            parser: our_parser,
            query: query,
            sender: sender,
        }
    }
}

impl<'a> ParallelVisitor for Visitor<'a> {
    fn visit(&mut self, dir_entry_result: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        match dir_entry_result {
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

                let tree = match self.parser.parse(&source, None) {
                    Some(t) => t,
                    None => {
                        eprintln!("Couldn't parse source of {:?}", dir_entry.path());
                        return WalkState::Quit;
                    }
                };

                let matches = QueryCursor::new()
                    // TODO: what's this third argument? It's called `text_callback` in the docs?
                    .matches(&self.query, tree.root_node(), |_| [])
                    .flat_map(|query_match| query_match.captures)
                    .map(|capture| {
                        capture
                            .node
                            .utf8_text(source.as_ref())
                            .map(|capture_source| Match {
                                path: dir_entry.path().to_path_buf(),
                                position: Point(capture.node.start_position()),
                                source: String::from(capture_source),
                            })
                    })
                    .collect::<Result<Vec<Match>, Utf8Error>>();

                match matches {
                    Ok(matches) => {
                        for match_ in matches {
                            if let Err(err) = self.sender.send(match_) {
                                eprintln!("Couldn't send a match: {:#?}", err);
                                return WalkState::Quit;
                            }
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
        }
    }
}

// dealing with queries

fn get_query(pattern: &str) -> Result<tree_sitter::Query, QueryError> {
    let language = language_elm();
    let query = Query::new(language, &pattern).map_err(QueryError::QueryError)?;

    // I want people to be able to write things like `(import_clause)` to match
    // the whole string, but tree-sitter will return an empty match in this
    // case since there are no captures. Easy, we just add an overall capture
    // group if there are none defined and there's only one pattern. There may
    // be more cases where this is appropriate, but I don't know about them yet!
    if query.pattern_count() == 1 && query.capture_names().is_empty() {
        Query::new(language, &(pattern.to_owned() + "@query")).map_err(QueryError::QueryError)
    } else {
        Ok(query)
    }
}

#[derive(Debug)]
enum QueryError {
    QueryError(tree_sitter::QueryError),
}

// output formats

#[derive(Debug)]
enum Format {
    Lines,
    JSON,
}

impl FromStr for Format {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lines" => Ok(Format::Lines),
            "json" => Ok(Format::JSON),
            _ => Err(FormatError::InvalidFormatString),
        }
    }
}

#[derive(Debug)]
enum FormatError {
    InvalidFormatString,
}

impl Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::InvalidFormatString => {
                write!(f, "valid values are \"lines\" and \"json\".")
            }
        }
    }
}

struct Formatter<'a> {
    format: Format,
    gatherer: Gatherer<'a>,
    matches: Vec<Match>,
}

impl<'a> Formatter<'a> {
    fn new(format: Format, gatherer: Gatherer<'a>) -> Formatter<'a> {
        Formatter {
            format: format,
            gatherer: gatherer,
            matches: Vec::new(),
        }
    }

    fn format(mut self) {
        // Before we can receive messages, we need to drop the original sender
        // channel so that the gathering will terminate once all the visitor
        // threads have finished visiting.
        //
        // I don't like the knowledge this has of gatherer's innards, but I
        // suppose it's OK... and I can't find another way to do it that both
        // compiles and works :\
        drop(self.gatherer.sender);

        match self.format {
            Format::Lines => {
                for match_ in self.gatherer.receiver {
                    println!(
                        "{}:{}:{}:{}",
                        match_.path.to_str().unwrap(), // TODO: no panicking!
                        match_.position.0.row,
                        match_.position.0.column,
                        match_.source
                    )
                }
            }

            Format::JSON => {
                for match_ in self.gatherer.receiver {
                    self.matches.push(match_)
                }

                println!("{}", serde_json::to_string(&self.matches).unwrap()); // TODO: no panicking!
            }
        }
    }
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
