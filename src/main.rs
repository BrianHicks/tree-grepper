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
        .for_each(|p| println!("{:?}", p));

    Ok(())
}

fn build_walker(opts: &Opts) -> Result<ignore::Walk> {
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
