use anyhow::{bail, Error, Result};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Language {
    Elm,
}

impl Language {
    fn all() -> Vec<Language> {
        vec![Language::Elm]
    }

    fn language(&self) -> tree_sitter::Language {
        match self {
            Language::Elm => language_elm(),
        }
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "elm" => Ok(Language::Elm),
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
            Language::Elm => f.write_str("elm"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_str_reflects_from_str() {
        // TODO: how do we aggregate failures here instead of failing early if
        // one doesn't match?
        Language::all()
            .into_iter()
            .for_each(|lang| assert_eq!(Language::from_str(&lang.to_string()).unwrap(), lang))
    }
}

extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

fn language_elm() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}
