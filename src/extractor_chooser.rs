use crate::extractor::Extractor;
use anyhow::{bail, Context, Result};
use ignore::types::{Types, TypesBuilder};
use ignore::DirEntry;
use std::collections::HashMap;

pub trait ExtractorChooser {
    // TODO: might need to return a borrow in the Option
    fn extractor_for(&self, entry: &DirEntry) -> Option<Extractor>;
}

pub struct MultipleChoices<'extractor> {
    matcher: Types,
    extractors: HashMap<String, &'extractor Extractor>,
}

impl<'extractor> MultipleChoices<'extractor> {
    pub fn from_extractors(extractors: &Vec<Extractor>) -> Result<MultipleChoices> {
        let mut types_builder = TypesBuilder::new();
        types_builder.add_defaults();

        let mut names_to_extractors = HashMap::with_capacity(extractors.len());

        for extractor in extractors {
            let name = extractor.language().to_string();
            types_builder.select(&name);

            // a little reminder: insert returns the old value if the key was
            // already present
            if names_to_extractors.insert(name, extractor).is_some() {
                bail!("got a duplicate query. This should not have happened. Please report it!")
            }
        }

        Ok(MultipleChoices {
            matcher: types_builder
                .build()
                .context("could not build a filetype matcher using provided languages")?,
            extractors: names_to_extractors,
        })
    }
}

impl<'extractor> ExtractorChooser for MultipleChoices<'extractor> {
    fn extractor_for(&self, entry: &DirEntry) -> Option<Extractor> {
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(true);
        // if is_dir {
        //     return None;
        // }

        let matched = self.matcher.matched(entry.path(), is_dir);
        println!("{:?}", matched);

        // if !matched.is_whitelist() {
        //     return None;
        // }

        // if let Some(_) = matched
        //     .inner()
        //     .and_then(|glob| glob.file_type_def())
        //     .and_then(|def| Language::from_str(def.name()).ok())
        // {
        //     todo!("probably all this should go in a matcher struct of some kind")
        // }

        None
    }
}
