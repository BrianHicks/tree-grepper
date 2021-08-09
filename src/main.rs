use anyhow::{bail, Result};
use clap::{crate_authors, crate_version, Clap};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::path::PathBuf;

mod language;
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

    walker(&opts)?
        .par_bridge()
        .for_each(|entry| println!("{:?}", entry));

    Ok(())
}

fn walker(opts: &Opts) -> Result<ignore::Walk> {
    let builder = match opts.paths.split_first() {
        Some((first, rest)) => {
            let mut builder = ignore::WalkBuilder::new(first);
            for path in rest {
                builder.add(path);
            }

            builder
        }
        None => bail!("I need at least one file or directory to walk!"),
    };

    // TODO: git ignore, file matching, et cetera

    Ok(builder.build())
}
