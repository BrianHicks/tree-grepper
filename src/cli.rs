use crate::extractor::Extractor;
use crate::extractor_chooser::ExtractorChooser;
use crate::language::Language;
use anyhow::{bail, Context, Result};
use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub struct Opts {
    pub extractors: Vec<Extractor>,
    pub paths: Vec<PathBuf>,
    pub git_ignore: bool,
}

impl Opts {
    pub fn from_args() -> Result<Opts> {
        // I'm not super happy with this! I would love for LANGUAGE and QUERY to
        // be taken positionally when there is just one so we don't always have
        // to specify `-q`. However, I also want to get working on the rest of
        // the program so I'm dropping the requirement for now by making `-q`
        // required. I think that's an OK tradeoff until I can figure something
        // else better because it'll be backwards compatible with the scheme
        // I outlined above.
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
                .multiple(true)
        )
        .arg(
            Arg::new("no-gitignore")
                .long("no-gitignore")
                .about("don't use git's ignore and exclude files to filter files")
        )
        .arg(
            Arg::new("PATHS")
                .default_value(".")
                .about("places to search for matches")
                .multiple(true)
        )
        .get_matches();

        Ok(Opts {
            extractors: Opts::extractors(&matches)?,
            paths: Opts::paths(&matches)?,
            git_ignore: !matches.is_present("no-gitignore"),
        })
    }

    fn extractors(matches: &ArgMatches) -> Result<Vec<Extractor>> {
        let values = match matches.values_of("additional-query") {
            Some(values) => values,
            None => bail!("queries were required but not provided. This indicates an internal error and you should report it!"),
        };

        // the most common case is going to be one query, so let's allocate
        // that immediately...
        let mut query_strings: HashMap<Language, String> = HashMap::with_capacity(1);

        // If you have two tree-sitter queries `(one)` and `(two)`, you can
        // join them together in a single string like `(one)(two)`. In that
        // case, the resulting query will act like an OR and match any of the
        // queries inside. Doing this automatically gives us an advantage:
        // for however many queries we get on the command line, we will only
        // ever have to run one per file, since we can combine them and you
        // can't specify queries across multiple languages! Nobody should ever
        // notice, except that they won't see as much of a slowdown for adding
        // new queries to an invocation as they might expect. (Well, hopefully!)
        for (raw_lang, raw_query) in values.tuples() {
            let lang = Language::from_str(raw_lang).context("could not parse language")?;

            if let Some(existing) = query_strings.get_mut(&lang) {
                existing.push_str(raw_query);
            } else {
                query_strings.insert(lang, String::from(raw_query));
            }
        }

        let mut out = Vec::with_capacity(query_strings.len());
        for (lang, raw_query) in query_strings {
            let query = lang
                .parse_query(&raw_query)
                .context("could not parse (combined) query")?;

            out.push(Extractor::new(lang, query))
        }

        Ok(out)
    }

    fn paths(matches: &ArgMatches) -> Result<Vec<PathBuf>> {
        match matches.values_of("PATHS") {
            Some(values) =>
                values
                    .map(|raw_path| PathBuf::from_str(raw_path).with_context(|| format!("could not parse a path from {}", raw_path)))
                    .collect(),

            None => bail!("at least one path was required but not provided. This indicates an internal errors and you should report it!"),
        }
    }

    pub fn extractor_chooser(&self) -> Result<ExtractorChooser> {
        ExtractorChooser::from_extractors(&self.extractors)
    }
}
