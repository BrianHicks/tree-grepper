use anyhow;
use std::process;
use thiserror::Error;
use tree_sitter;

fn main() {
    if let Err(err) = real_main() {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

fn real_main() -> anyhow::Result<()> {
    let _parser = elm_parser();
    Ok(())
}

// tree-sitter setup

extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

fn language_elm() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}

fn elm_parser() -> anyhow::Result<tree_sitter::Parser> {
    let mut parser = tree_sitter::Parser::new();

    parser
        .set_language(language_elm())
        .map_err(LanguageError::LanguageError)?;

    Ok(parser)
}

#[derive(Error, Debug)]
enum LanguageError {
    #[error("tree sitter language error: {0:?}")]
    LanguageError(tree_sitter::LanguageError),
}
