use anyhow::{bail, Context, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};

mod cli;
mod language;

use cli::Opts;

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let opts = Opts::from_args()
        .context("couldn't get a valid configuration from the command-line options")?;

    build_walker(&opts)
        .context("couldn't build a filesystem walker")?
        .par_bridge()
        .for_each(|entry_result| match entry_result {
            Ok(entry) => println!("{:?}", entry.path()),
            Err(err) => println!("{:?}", err),
        });

    Ok(())
}

fn build_walker(opts: &Opts) -> Result<ignore::Walk> {
    let mut builder = match opts.paths.split_first() {
        Some((first, rest)) => {
            let mut builder = ignore::WalkBuilder::new(first);
            for path in rest {
                builder.add(path);
            }

            builder
        }
        None => bail!("I need at least one file or directory to walk!"),
    };

    Ok(builder
        .git_ignore(opts.git_ignore)
        .git_exclude(opts.git_ignore)
        .git_global(opts.git_ignore)
        .types(opts.filetype_matcher()?)
        .build())
}
