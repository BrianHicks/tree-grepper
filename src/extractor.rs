use crate::language::Language;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query};

#[derive(Debug)]
pub struct Extractor {
    language: Language,
    query: Query,
}

impl Extractor {
    pub fn new(language: Language, query: Query) -> Extractor {
        Extractor { language, query }
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn extract_from_file(&self, path: &Path) -> Result<()> {
        let source = fs::read_to_string(path)
            .with_context(|| format!("could not read {}", path.display()))?;

        // TODO: this is going to allocate a new parser for every single matched
        // file. Is this something that we want? Parsers are not thread-safe,
        // so avoiding this might mean rewriting the core of the program away
        // from being in Rayon's parallel iterator abstraction.
        //
        // Also, do we need to avoid calling self.language.language() repeatedly?
        let mut parser = Parser::new();
        parser
            .set_language(self.language.language())
            .context("could not set language")?;

        let tree = parser.parse(source, None);

        println!("{:?}", tree);

        Ok(())
    }
}
