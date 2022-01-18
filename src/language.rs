use anyhow::{anyhow, bail, Error, Result};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Language {
    Cpp,
    Elixir,
    Elm,
    Haskell,
    JavaScript,
    Php,
    Ruby,
    Rust,
    TypeScript,
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![
            Language::Cpp,
            Language::Elixir,
            Language::Elm,
            Language::Haskell,
            Language::JavaScript,
            Language::Php,
            Language::Ruby,
            Language::Rust,
            Language::TypeScript,
        ]
    }

    pub fn language(&self) -> tree_sitter::Language {
        unsafe {
            match self {
                Language::Cpp => tree_sitter_cpp(),
                Language::Elixir => tree_sitter_elixir(),
                Language::Elm => tree_sitter_elm(),
                Language::Haskell => tree_sitter_haskell(),
                Language::JavaScript => tree_sitter_javascript(),
                Language::Php => tree_sitter_php(),
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
            Language::Cpp => "cpp",
            Language::Elixir => "elixir",
            Language::Elm => "elm",
            Language::Haskell => "haskell",
            Language::JavaScript => "js",
            Language::Php => "php",
            Language::Ruby => "ruby",
            Language::Rust => "rust",
            Language::TypeScript => "ts",
        }
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "cpp" => Ok(Language::Cpp),
            "elixir" => Ok(Language::Elixir),
            "elm" => Ok(Language::Elm),
            "haskell" => Ok(Language::Haskell),
            "javascript" => Ok(Language::JavaScript),
            "php" => Ok(Language::Php),
            "ruby" => Ok(Language::Ruby),
            "rust" => Ok(Language::Rust),
            "typescript" => Ok(Language::TypeScript),
            _ => bail!(
                "unknown language {}. Try one of: {}",
                s,
                Language::all()
                    .into_iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Language::Cpp => f.write_str("cpp"),
            Language::Elixir => f.write_str("elixir"),
            Language::Elm => f.write_str("elm"),
            Language::Haskell => f.write_str("haskell"),
            Language::JavaScript => f.write_str("javascript"),
            Language::Php => f.write_str("php"),
            Language::Ruby => f.write_str("ruby"),
            Language::Rust => f.write_str("rust"),
            Language::TypeScript => f.write_str("typescript"),
        }
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
        Language::all()
            .into_iter()
            .for_each(|lang| assert_eq!(Language::from_str(&lang.to_string()).unwrap(), lang))
    }

    #[test]
    fn parse_query_smoke_test() {
        assert_eq!(true, Language::Elm.parse_query("(_)").is_ok());
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
    fn tree_sitter_cpp() -> tree_sitter::Language;
    fn tree_sitter_elixir() -> tree_sitter::Language;
    fn tree_sitter_elm() -> tree_sitter::Language;
    fn tree_sitter_haskell() -> tree_sitter::Language;
    fn tree_sitter_javascript() -> tree_sitter::Language;
    fn tree_sitter_php() -> tree_sitter::Language;
    fn tree_sitter_ruby() -> tree_sitter::Language;
    fn tree_sitter_rust() -> tree_sitter::Language;
    fn tree_sitter_typescript() -> tree_sitter::Language;
}
