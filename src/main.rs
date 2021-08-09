use anyhow::{bail, Context, Result};
use clap::{crate_authors, crate_version, App, Arg};
use itertools::Itertools;
use std::str::FromStr;
// use rayon::iter::{ParallelBridge, ParallelIterator};
// use std::path::PathBuf;

mod language;
use language::Language;

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    get_opts().context("couldn't get a valid configuration from the command-line options")?;

    Ok(())
}

fn get_opts() -> Result<()> {
    // I'm not super happy with this! I would love for LANGUAGE and QUERY to
    // be taken positionally when there is just one so we don't always have
    // to specify `-q`. However, I also want to get working on the rest of the
    // program so I'm dropping the requirement for now by making `-q` required. I
    // think that's an OK tradeoff until I can figure something else better
    // because it'll be backwards compatible with the scheme I outlined above.
    //
    // Check
    // https://users.rust-lang.org/t/grep-like-argument-parsing-with-clap/63392
    // for where I asked about this in public.
    //
    // TODO: would the above be better in a lazy_static?
    let matches = App::new("tree-grepper")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("additional-query")
                .short('q')
                .long("query")
                .about("a language and query to perform")
                .long_about("a language and query to perform. See https://tree-sitter.github.io for information on writing queries. TODO: add a mode to list languages.")
                .number_of_values(2)
                .value_names(&["LANGUAGE", "QUERY"])
                .required(true)
                .multiple(true),
        )
        .arg(Arg::new("PATHS").default_value(".").multiple(true))
        .get_matches();

    // queries
    let queries = match matches.values_of("additional-query") {
        Some(values) => values.tuples().enumerate().map(|(nth, (raw_lang, raw_query))| {
            let lang = Language::from_str(raw_lang).with_context(|| format!("could not parse query #{}", nth + 1))?;
            let query = lang.parse_query(raw_query).with_context(|| format!("could not parse query #{}", nth + 1))?;

            Ok((lang, query))
        }).collect::<Result<Vec<(Language, tree_sitter::Query)>>>()?,
        None => bail!("additional-query was required. This is probably an internal error and you should report it!"),
    };

    println!("{:?}", queries);

    // files
    println!("{:?}", matches);

    Ok(())
}

// fn walker(opts: &Opts) -> Result<ignore::Walk> {
//     let builder = match opts.paths.split_first() {
//         Some((first, rest)) => {
//             let mut builder = ignore::WalkBuilder::new(first);
//             for path in rest {
//                 builder.add(path);
//             }

//             builder
//         }
//         None => bail!("I need at least one file or directory to walk!"),
//     };

//     // TODO: git ignore, file matching, et cetera

//     Ok(builder.build())
// }
