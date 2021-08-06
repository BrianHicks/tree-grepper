use clap::{crate_authors, crate_version, Clap};
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author=crate_authors!())]
struct Opts {
    /// The tree-sitter s-expression query to search for. See the tree sitter
    /// docs on how to make these at https://tree-sitter.github.io
    pattern: String,

    /// Paths to look for files. Can be files, directories, or a combination.
    #[clap(default_value = ".", parse(from_os_str))]
    paths: Vec<PathBuf>,
}

fn main() {
    let opts = Opts::parse();

    println!("{:?}", opts);
}
