use anyhow::{bail, Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io;
use tree_sitter::Parser;

mod cli;
mod extractor;
mod extractor_chooser;
mod language;

use cli::{Format, Opts};

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

    let extracted_files = items
        .par_iter()
        .filter_map(|entry| {
            chooser
                .extractor_for(entry)
                .map(|extractor| (entry, extractor))
        })
        .map_init(
            || Parser::new(),
            |parser, (entry, extractor)| extractor.extract_from_file(entry.path(), parser),
        )
        .filter_map(|result_containing_option| match result_containing_option {
            Ok(None) => None,
            Ok(Some(extraction)) => Some(Ok(extraction)),
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<extractor::ExtractedFile>>>()
        .context("couldn't extract matches from files")?;

    match opts.format {
        Format::Lines => {
            for extracted_file in extracted_files {
                print!("{}", extracted_file);
            }
        }

        Format::JSON => {
            serde_json::to_writer(io::stdout(), &extracted_files)
                .context("could not write JSON output")?;
        }
    }

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
