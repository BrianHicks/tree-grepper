use anyhow::Result;
use clap::{crate_authors, crate_version, Clap};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::path::PathBuf;

mod files;
mod language;

use files::Files;
use language::Language;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author=crate_authors!())]
struct Opts {
    /// What language are we matching against?
    language: Language,

    /// The tree-sitter s-expression query to search for. See the tree sitter
    /// docs on how to make these at https://tree-sitter.github.io
    pattern: String,

    /// Paths to look for files. Can be files, directories, or a combination.
    #[clap(default_value = ".", parse(from_os_str))]
    paths: Vec<PathBuf>,
}

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let opts = Opts::parse();

    Files::new(opts.paths)
        .par_bridge()
        .for_each(|path| println!("{:?}", path));

    Ok(())
}
