use crate::language::Language;
use anyhow::{bail, Context, Result};
use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use ignore::types;
use itertools::Itertools;
use std::path::PathBuf;
use std::str::FromStr;
use tree_sitter::Query;

#[derive(Debug)]
pub struct Opts {
    pub queries: Vec<(Language, Query)>,
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
            queries: Opts::queries(&matches)?,
            paths: Opts::paths(&matches)?,
            git_ignore: !matches.is_present("no-gitignore"),
        })
    }

    fn queries(matches: &ArgMatches) -> Result<Vec<(Language, Query)>> {
        let values = match matches.values_of("additional-query") {
            Some(values) => values,
            None => bail!("queries were required but not provided. This indicates an internal error and you should report it!"),
        };

        values
            .tuples()
            .enumerate()
            .map(|(nth, (raw_lang, raw_query))| {
                let lang = Language::from_str(raw_lang)
                    .with_context(|| format!("could not parse query #{}", nth + 1))?;
                let query = lang
                    .parse_query(raw_query)
                    .with_context(|| format!("could not parse query #{}", nth + 1))?;

                Ok((lang, query))
            })
            .collect()
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

    pub fn filetype_matcher(&self) -> Result<types::Types> {
        let mut types_builder = types::TypesBuilder::new();
        types_builder.add_defaults();
        for (lang, _) in &self.queries {
            types_builder.select(&lang.to_string());
        }

        types_builder
            .build()
            .context("could not build a filetype matcher")
    }
}
