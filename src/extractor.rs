use crate::language::Language;
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
}
