use anyhow;
use clap::Clap;
use std::path::PathBuf;
use std::process;
use thiserror::Error;
use tree_sitter;
use walkdir::WalkDir;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Brian Hicks <brian@brianthicks.com>")]
struct Opts {
    /// Pattern to search for.
    pattern: String,

    /// Paths to look for files. Can be files, directories, or a mix of both.
    #[clap(default_value = ".", parse(from_os_str))]
    paths: Vec<PathBuf>,

    /// Follow symlinks
    #[clap(short('f'), long("follow"))]
    follow_links: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    if let Err(err) = real_main(opts) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

fn real_main(opts: Opts) -> anyhow::Result<()> {
    opts.paths
        .iter()
        .flat_map(|path| WalkDir::new(path).follow_links(opts.follow_links))
        .map(|e| println!("{:?}", e))
        .collect::<Vec<()>>();

    let _parser = elm_parser();
    Ok(())
}

// tree-sitter setup

extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

fn language_elm() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}

fn elm_parser() -> anyhow::Result<tree_sitter::Parser> {
    let mut parser = tree_sitter::Parser::new();

    parser
        .set_language(language_elm())
        .map_err(LanguageError::LanguageError)?;

    Ok(parser)
}

#[derive(Error, Debug)]
enum LanguageError {
    #[error("tree sitter language error: {0:?}")]
    LanguageError(tree_sitter::LanguageError),
}
