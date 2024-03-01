mod cli;
mod extractor;
mod extractor_chooser;
mod language;
mod tree_view;

use anyhow::{bail, Context, Result};
use cli::{Invocation, QueryFormat, QueryOpts, TreeOpts};
use crossbeam::channel;
use language::Language;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::env;
use std::fs;
use std::io::{self, BufWriter, Write};
use tree_sitter::Parser;

#[global_allocator]
static ALLOCATOR: bump_alloc::BumpAlloc = bump_alloc::BumpAlloc::new();

fn main() {
    let mut buffer = BufWriter::new(io::stdout());

    if let Err(error) = try_main(env::args().collect(), &mut buffer) {
        if let Some(err) = error.downcast_ref::<io::Error>() {
            // a broken pipe is totally normal and fine. It's what we get when
            // we pipe to something like `head` that only takes a certain number
            // of lines.
            if err.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }

        if let Some(clap_error) = error.downcast_ref::<clap::Error>() {
            // Clap errors (--help or misuse) are already well-formatted,
            // so we don't have to do any additional work.
            eprint!("{}", clap_error);
            if clap_error.kind() == clap::error::ErrorKind::DisplayHelp
                || clap_error.kind() == clap::error::ErrorKind::DisplayVersion
            {
                std::process::exit(0);
            }
        } else {
            eprintln!("{:?}", error);
        }

        std::process::exit(1);
    }

    buffer.flush().expect("failed to flush buffer!");
}

fn try_main(args: Vec<String>, out: impl Write) -> Result<()> {
    let invocation = Invocation::from_args(args)
        .context("couldn't get a valid configuration from the command-line options")?;

    match invocation {
        Invocation::DoQuery(query_opts) => {
            do_query(query_opts, out).context("couldn't perform the query")
        }
        Invocation::ShowLanguages => {
            show_languages(out).context("couldn't show the list of languages")
        }
        Invocation::ShowTree(tree_opts) => {
            show_tree(tree_opts, out).context("couldn't show the tree")
        }
    }
}

fn show_languages(mut out: impl Write) -> Result<()> {
    for language in Language::all() {
        writeln!(out, "{}", language).context("couldn't print a language")?;
    }

    Ok(())
}

fn show_tree(opts: TreeOpts, out: impl Write) -> Result<()> {
    let source = fs::read_to_string(opts.path).context("could not read target file")?;

    let mut parser = Parser::new();
    parser
        .set_language(&opts.language.language())
        .context("could not set language")?;

    let tree = parser
        .parse(&source, None)
        .context("could not parse tree")?;

    tree_view::tree_view(&tree, source.as_bytes(), out)
}

fn do_query(opts: QueryOpts, mut out: impl Write) -> Result<()> {
    // You might think "why not use ParallelBridge here?" Well, the quick answer
    // is that I benchmarked it and having things separated here and handling
    // their own errors actually speeds up this part of the code by like 20%!
    let items: Vec<ignore::DirEntry> =
        find_files(&opts).context("had a problem while walking the filesystem")?;

    let chooser = opts
        .extractor_chooser()
        .context("couldn't construct a filetype matcher")?;

    let mut extracted_files = items
        .par_iter()
        .filter_map(|entry| {
            chooser
                .extractor_for(entry)
                .map(|extractor| (entry, extractor))
        })
        .map_init(Parser::new, |parser, (entry, extractor)| {
            extractor
                .extract_from_file(entry.path(), parser)
                .with_context(|| {
                    format!("could not extract matches from {}", entry.path().display())
                })
        })
        .filter_map(|result_containing_option| match result_containing_option {
            Ok(None) => None,
            Ok(Some(extraction)) => Some(Ok(extraction)),
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<extractor::ExtractedFile>>>()
        .context("couldn't extract matches from files")?;

    if opts.sort {
        extracted_files.sort()
    }

    match opts.format {
        QueryFormat::Lines => {
            for extracted_file in extracted_files {
                write!(out, "{}", extracted_file).context("could not write lines")?;
            }
        }

        QueryFormat::Json => {
            serde_json::to_writer(out, &extracted_files).context("could not write JSON output")?;
        }

        QueryFormat::JsonLines => {
            for extracted_file in extracted_files {
                writeln!(
                    out,
                    "{}",
                    serde_json::to_string(&extracted_file)
                        .context("could not write JSON output")?
                )
                .context("could not write line")?;
            }
        }

        QueryFormat::PrettyJson => {
            serde_json::to_writer_pretty(out, &extracted_files)
                .context("could not write JSON output")?;
        }
    }

    Ok(())
}

fn find_files(opts: &QueryOpts) -> Result<Vec<ignore::DirEntry>> {
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

    let (root_sender, receiver) = channel::unbounded();

    builder
        .git_ignore(opts.git_ignore)
        .git_exclude(opts.git_ignore)
        .git_global(opts.git_ignore)
        .build_parallel()
        .run(|| {
            let sender = root_sender.clone();
            Box::new(move |entry_result| match entry_result {
                Ok(entry) => match sender.send(entry) {
                    Ok(()) => ignore::WalkState::Continue,
                    Err(_) => ignore::WalkState::Quit,
                },
                Err(_) => ignore::WalkState::Quit,
            })
        });

    drop(root_sender);

    Ok(receiver.iter().collect())
}
