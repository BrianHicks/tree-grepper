mod cli;
mod extractor;
mod extractor_chooser;
mod language;

use anyhow::{bail, Context, Result};
use cli::{Format, Opts};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::env;
use std::io::{self, Write};
use tree_sitter::Parser;

#[global_allocator]
static ALLOCATOR: bump_alloc::BumpAlloc = bump_alloc::BumpAlloc::new();

fn main() {
    if let Err(error) = try_main(env::args().collect(), &io::stdout()) {
        // Clap errors (--help or misuse) are already well-formatted, so we
        // don't have to do any additional work.
        if let Some(clap_error) = error.downcast_ref::<clap::Error>() {
            eprintln!("{}", clap_error);
        } else {
            eprintln!("{:?}", error);
        }
        std::process::exit(1);
    }
}

fn try_main(args: Vec<String>, mut out: impl Write) -> Result<()> {
    let opts = Opts::from_args(args)
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

    let mut extracted_files = items
        .par_iter()
        .filter_map(|entry| {
            chooser
                .extractor_for(entry)
                .map(|extractor| (entry, extractor))
        })
        .map_init(Parser::new, |parser, (entry, extractor)| {
            extractor.extract_from_file(entry.path(), parser)
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
        Format::Lines => {
            for extracted_file in extracted_files {
                write!(out, "{}", extracted_file).context("could not write lines")?;
            }
        }

        Format::Json => {
            serde_json::to_writer(out, &extracted_files).context("could not write JSON output")?;
        }

        Format::PrettyJson => {
            serde_json::to_writer_pretty(out, &extracted_files)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn call(args: &[&str]) -> String {
        let mut bytes = Vec::new();
        try_main(
            args.iter().map(|s| s.to_string()).collect(),
            Box::new(&mut bytes),
        )
        .unwrap();

        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn lines_output() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "elm",
            "(import_clause)",
            "-f",
            "lines",
            "--sort",
            "vendor/tree-sitter-elm",
        ]))
    }

    #[test]
    fn json_output() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "elm",
            "(import_clause)",
            "-f",
            "json",
            "--sort",
            "vendor/tree-sitter-elm",
        ]))
    }

    #[test]
    fn pretty_json_output() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "elm",
            "(import_clause)",
            "--format=pretty-json",
            "--sort",
            "vendor/tree-sitter-elm",
        ]))
    }

    // All languages should have a test that just spits out their entire node
    // tree. We use this to know about changes in the vendored parsers!

    #[test]
    fn all_elm() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "elm",
            "(_)",
            "--format=pretty-json",
            "--sort",
            "vendor/tree-sitter-elm",
        ]))
    }

    #[test]
    fn all_haskell() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "haskell",
            "(_)",
            "--format=pretty-json",
            "--sort",
            "vendor/tree-sitter-haskell",
        ]))
    }

    #[test]
    fn all_javascript() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "javascript",
            "(_)",
            "--format=pretty-json",
            "--sort",
            // note that this doesn't include the entire vendor
            // directory. tree-sitter-javascript vendors a couple of libraries
            // to test things and it makes this test run unacceptably long. I
            // think the slowdown is due to the diffing step; the tree-grepper
            // code completes in a reasonable amount of time.
            "vendor/tree-sitter-javascript/test",
        ]))
    }

    #[test]
    fn all_ruby() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "ruby",
            "(_)",
            "--format=pretty-json",
            "--sort",
            "vendor/tree-sitter-ruby",
        ]))
    }

    #[test]
    fn all_rust() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "rust",
            "(_)",
            "--format=pretty-json",
            "--sort",
            "vendor/tree-sitter-rust",
        ]))
    }

    #[test]
    fn all_typescript() {
        insta::assert_snapshot!(call(&[
            "tree-grepper",
            "-q",
            "typescript",
            "(_)",
            "--format=pretty-json",
            "--sort",
            // similar to JavaScript, there is one particular test file in this
            // grammar that's *huge*. It seems to be a comprehensive listing of
            // all the typescript syntax, maybe? Regardless, it makes this test
            // unacceptably slow, so we just look at one particular file. If
            // we see uncaught regressions in this function, we probably will
            // make our own test file with the things we care about.
            "vendor/tree-sitter-typescript/typescript/test.ts",
        ]))
    }
}
