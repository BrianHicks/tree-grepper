use clap::Clap;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

#[derive(Clap, Debug)]
/// Dump a file to an s-expression so you can see what things you can
/// search against.
struct Opts {
    /// Which file should we dump?
    file: PathBuf,

    /// What language should we require? (This is a required flag, but may be
    /// optional later if we add something that can determine file type or just
    /// try them all in sequence.)
    #[clap(short('l'), long)]
    language: Language,
}

fn main() {
    let opts = Opts::parse();

    let mut parser = match opts.language.parser() {
        Ok(p) => p,
        Err(err) => {
            eprintln!(
                "Could not load a parser. This is probably an internal error. Details: {}",
                err
            );
            process::exit(1);
        }
    };

    let source = match fs::read(&opts.file) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Could not read the source file: {}", err);
            process::exit(1);
        }
    };

    let tree = match parser.parse(&source, None) {
        Some(t) => t,
        None => {
            eprintln!("Couldn't parse source of {}", opts.file.display());
            process::exit(1);
        }
    };

    println!("{:}", tree.root_node().to_sexp());
}

// tree-sitter setup
//
// TODO: this is all duplicated from tree-grepper.rs. Once I figure out how
// the module system works with separate binaries I should go back and pull
// this into a shared crate.

#[derive(Debug)]
enum Language {
    Elm,
    Ruby,
    JavaScript,
}

impl FromStr for Language {
    type Err = LanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "elm" => Ok(Language::Elm),
            "ruby" => Ok(Language::Ruby),
            "javascript" => Ok(Language::JavaScript),
            _ => Err(LanguageError::UnknownLanguage),
        }
    }
}

impl Language {
    fn parser(&self) -> Result<tree_sitter::Parser, tree_sitter::LanguageError> {
        let mut parser = tree_sitter::Parser::new();

        parser.set_language(match self {
            Language::Elm => language_elm(),
            Language::Ruby => language_ruby(),
            Language::JavaScript => language_javascript(),
        })?;

        Ok(parser)
    }
}

extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
    fn tree_sitter_ruby() -> tree_sitter::Language;
    fn tree_sitter_javascript() -> tree_sitter::Language;
}

fn language_elm() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}
fn language_ruby() -> tree_sitter::Language {
    unsafe { tree_sitter_ruby() }
}

fn language_javascript() -> tree_sitter::Language {
    unsafe { tree_sitter_javascript() }
}

#[derive(Debug)]
enum LanguageError {
    UnknownLanguage,
}

impl Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageError::UnknownLanguage => {
                write!(f, "Unknown language. Try one of \"elm\" or \"ruby\"")
            }
        }
    }
}
