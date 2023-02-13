use anyhow::{anyhow, Error, Result};
use std::str::FromStr;
use strum::{IntoEnumIterator, VariantNames};
use strum_macros::{Display, EnumIter, EnumVariantNames, FromRepr};

#[derive(Display, FromRepr, EnumIter, EnumVariantNames, PartialEq, Eq, Hash, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    C,
    Cpp,
    Elixir,
    Elm,
    Go,
    Haskell,
    Java,
    JavaScript,
    Markdown,
    Nix,
    Php,
    Python,
    Ruby,
    Rust,
    TypeScript,
}

impl Language {
    pub fn all() -> Vec<Language> {
        Language::iter().collect()
    }

    pub fn language(&self) -> tree_sitter::Language {
        unsafe {
            match self {
                Language::C => tree_sitter_c(),
                Language::Cpp => tree_sitter_cpp(),
                Language::Elixir => tree_sitter_elixir(),
                Language::Elm => tree_sitter_elm(),
                Language::Go => tree_sitter_go(),
                Language::Haskell => tree_sitter_haskell(),
                Language::Java => tree_sitter_java(),
                Language::JavaScript => tree_sitter_javascript(),
                Language::Markdown => tree_sitter_markdown(),
                Language::Nix => tree_sitter_nix(),
                Language::Php => tree_sitter_php(),
                Language::Python => tree_sitter_python(),
                Language::Ruby => tree_sitter_ruby(),
                Language::Rust => tree_sitter_rust(),
                Language::TypeScript => tree_sitter_typescript(),
            }
        }
    }

    pub fn parse_query(&self, raw: &str) -> Result<tree_sitter::Query> {
        tree_sitter::Query::new(self.language(), raw).map_err(|err| anyhow!("{}", err))
    }

    pub fn name_for_types_builder(&self) -> &str {
        match self {
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Elixir => "elixir",
            Language::Elm => "elm",
            Language::Go => "go",
            Language::Haskell => "haskell",
            Language::Java => "java",
            Language::JavaScript => "js",
            Language::Markdown => "markdown",
            Language::Nix => "nix",
            Language::Php => "php",
            Language::Python => "py",
            Language::Ruby => "ruby",
            Language::Rust => "rust",
            Language::TypeScript => "ts",
        }
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let languages = Language::VARIANTS;
        languages
            .binary_search(&s)
            .map(|idx| Language::from_repr(idx).unwrap())
            .map_err(|_| {
                anyhow!(
                    "unknown language {}. Try one of: {}",
                    s,
                    languages.join(", ")
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_str_reflects_from_str() {
        // Note: this will hide results if there are multiple failures. It's
        // something that could be worked around but I don't think it is right
        // now. If it bothers you in the future, feel free to take a stab at it!
        Language::iter()
            .for_each(|lang| assert_eq!(Language::from_str(&lang.to_string()).unwrap(), lang))
    }

    #[test]
    fn parse_query_smoke_test() {
        Language::iter().for_each(|lang| assert!(lang.parse_query("(_)").is_ok()));
    }

    #[test]
    fn language_list_should_be_sorted() {
        use itertools::Itertools;
        // sorted elements needed by binary_search in FromStr
        // TODO: use is_sorted: https://github.com/rust-lang/rust/issues/53485
        assert!(Language::VARIANTS
            .iter()
            .tuple_windows()
            .all(|(a, b)| a <= b));
    }

    #[test]
    fn parse_query_problem() {
        // tree-grepper 1.0 just printed the error struct when problems like
        // this happened. This test is just here to make sure we take a slightly
        // friendlier approach for 2.0.
        assert_eq!(
            String::from("Query error at 1:2. Invalid node type node_that_doesnt_exist"),
            Language::Elm
                .parse_query("(node_that_doesnt_exist)")
                .unwrap_err()
                .to_string(),
        )
    }
}

extern "C" {
    fn tree_sitter_c() -> tree_sitter::Language;
    fn tree_sitter_cpp() -> tree_sitter::Language;
    fn tree_sitter_elixir() -> tree_sitter::Language;
    fn tree_sitter_elm() -> tree_sitter::Language;
    fn tree_sitter_go() -> tree_sitter::Language;
    fn tree_sitter_haskell() -> tree_sitter::Language;
    fn tree_sitter_java() -> tree_sitter::Language;
    fn tree_sitter_javascript() -> tree_sitter::Language;
    fn tree_sitter_markdown() -> tree_sitter::Language;
    fn tree_sitter_nix() -> tree_sitter::Language;
    fn tree_sitter_php() -> tree_sitter::Language;
    fn tree_sitter_python() -> tree_sitter::Language;
    fn tree_sitter_ruby() -> tree_sitter::Language;
    fn tree_sitter_rust() -> tree_sitter::Language;
    fn tree_sitter_typescript() -> tree_sitter::Language;
}
