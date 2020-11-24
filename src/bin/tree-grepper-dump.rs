use clap::Clap;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Clap, Debug)]
/// Dump a file to an s-expression so you can see what things you can
/// search against.
struct Opts {
    /// Which file should we dump?
    file: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    let mut parser = match parser(language_elm()) {
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

extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

fn language_elm() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}

fn parser(
    language: tree_sitter::Language,
) -> Result<tree_sitter::Parser, tree_sitter::LanguageError> {
    let mut parser = tree_sitter::Parser::new();

    parser.set_language(language)?;

    Ok(parser)
}
