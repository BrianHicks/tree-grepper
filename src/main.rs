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
        .filter_map(|entry_result| match entry_result {
            Ok(entry) => {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(true) {
                    None
                } else {
                    Some(Ok(entry))
                }
            }
            Err(err) => Some(Err(err).context("could not walk a path")),
        })
        // remove directories and read files
        // parse files according to query
        .for_each(|entry_result| match entry_result {
            Ok(entry) => println!("Read source: {:?}", entry),
            Err(err) => println!("Problem: {:?}", err),
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
        // TODO: profile to see if construction the filetype matcher twice is
        // expensive and maybe don't.
        .types(opts.filetype_matcher()?)
        .build())
}
