use anyhow::{self, Context};
use clap::Clap;
use ignore::{types, WalkBuilder, WalkState};
use std::path::PathBuf;
use std::process;
use thiserror::Error;
use tree_sitter::{self, Query};

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
    if let Err(err) = real_main(opts) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

fn real_main(opts: Opts) -> anyhow::Result<()> {
    let mut parser = parser(language_elm()).context("couldn't get the parser")?;

    // TODO: this error type has rich enough text to make a really nice error
    // message, but this implementation ends up pretty crappy. Make it better!
    let query = Query::new(language_elm(), &opts.pattern)
        .map_err(TreeSitterError::QueryError)
        .context("invalid pattern")?;

    // I *think* we should be OK to assume that there's at least one path in
    // this `opts.paths`, since there will be a default set above. This code
    // is a little incautious as a result, but a future refactor could break
    // it! Is there a better way? (e.g. making it impossible by construction
    // like `(a, Vec<a>)`?)
    let mut builder = WalkBuilder::new(opts.paths[0].clone());

    // argh! how do I iterate starting at index 1? Is this the right way?
    let mut idx = 1;
    while let Some(path) = opts.paths.get(idx) {
        builder.add(path);
        idx += 1;
    }

    let mut types_builder = types::TypesBuilder::new();
    types_builder
        .add("elm", "*.elm")
        .context("couldn't add Elm type")?;
    types_builder.select("elm");
    let types = types_builder.build().context("couldn't build types")?;

    builder
        .follow_links(opts.follow_links)
        .max_depth(opts.max_depth)
        .threads(opts.threads)
        .types(types)
        .build_parallel()
        .run(|| {
            Box::new(|path| {
                println!("{:?}", path);
                WalkState::Continue
            })
        });

    Ok(())
}

// tree-sitter setup

extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

fn language_elm() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}

fn parser(language: tree_sitter::Language) -> anyhow::Result<tree_sitter::Parser> {
    let mut parser = tree_sitter::Parser::new();

    parser
        .set_language(language)
        .map_err(TreeSitterError::LanguageError)?;

    Ok(parser)
}

#[derive(Error, Debug)]
enum TreeSitterError {
    #[error("tree sitter language error: {0:?}")]
    LanguageError(tree_sitter::LanguageError),

    #[error("problem with query: {0:?}")]
    QueryError(tree_sitter::QueryError),
}
