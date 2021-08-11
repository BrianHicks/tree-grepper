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

    let matcher = opts
        .filetype_matcher()
        .context("couldn't construct a filetype matcher")?;

    build_walker(&opts)
        .context("couldn't build a filesystem walker")?
        .par_bridge()
        .filter_map(|entry_result| match entry_result {
            Ok(entry) => {
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(true);
                if is_dir {
                    return None;
                }

                let matched = matcher.matched(entry.path(), is_dir);
                println!("{:?}", matched);

                if !matched.is_whitelist() {
                    return None;
                }

                Some(Ok(entry))
            }
            Err(err) => Some(Err(err).context("could not walk a path")),
        })
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
        .build())
}
