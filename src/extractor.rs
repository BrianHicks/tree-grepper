use crate::language::Language;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tree_sitter::Query;

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

        Ok(())
    }
}
