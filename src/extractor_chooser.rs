use crate::extractor::Extractor;
use anyhow::{bail, Context, Result};
use ignore::types::{Types, TypesBuilder};
use ignore::DirEntry;
use std::collections::HashMap;

pub struct ExtractorChooser<'extractor> {
    matcher: Types,
    extractors: HashMap<&'extractor str, &'extractor Extractor>,
}

impl<'extractor> ExtractorChooser<'extractor> {
    pub fn from_extractors(extractors: &[Extractor]) -> Result<ExtractorChooser> {
        let mut types_builder = TypesBuilder::new();
        types_builder.add_defaults();

        let mut names_to_extractors = HashMap::with_capacity(extractors.len());

        for extractor in extractors {
            let name = extractor.language().name_for_types_builder();
            types_builder.select(name);

            // a little reminder: insert returns the old value if the key was
            // already present
            if names_to_extractors.insert(name, extractor).is_some() {
                bail!("got a duplicate query. This should not have happened. Please report it!")
            }
        }

        Ok(ExtractorChooser {
            matcher: types_builder
                .build()
                .context("could not build a filetype matcher using provided extractors")?,
            extractors: names_to_extractors,
        })
    }

    pub fn extractor_for(&self, entry: &DirEntry) -> Option<&Extractor> {
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(true);
        let matched = self.matcher.matched(entry.path(), is_dir);

        if !matched.is_whitelist() {
            return None;
        }

        matched
            .inner()
            .and_then(|glob| glob.file_type_def())
            .and_then(|def| self.extractors.get(def.name()))
            .copied()
    }
}
