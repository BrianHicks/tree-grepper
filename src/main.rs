use anyhow::{bail, Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod cli;
mod extractor;
mod extractor_chooser;
mod language;

use cli::Opts;
use extractor_chooser::ExtractorChooser;

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let opts = Opts::from_args()
        .context("couldn't get a valid configuration from the command-line options")?;

    // You might think "why not use ParallelBridge here?" Well, the quick answer
    // is that I benchmarked it and having things separated here and handling
    // their own errors actually speeds up this part of the code by like 20%!
    let items: Vec<ignore::DirEntry> = build_walker(&opts)
        .context("couldn't build a filesystem walker")?
        .collect::<Result<Vec<ignore::DirEntry>, ignore::Error>>()
        .context("had a problem while walking the filesystem")?;

    let chooser = opts
        .extractor_chooser()
        .context("couldn't construct a filetype matcher")?;

    items
        .par_iter()
        .filter_map(|entry| chooser.extractor_for(entry))
        // parse files according to query
        .for_each(|entry| println!("Read source: {:?}", entry));

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
