use std::path::PathBuf;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dir: PathBuf = ["vendor", "tree-sitter-elm", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/parser.c");
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-elm");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/scanner.cc");
    cc::Build::new()
        .include(&dir)
        .cpp(true)
        .file(dir.join("scanner.cc"))
        .compile("tree_sitter_elm_scanner")
}
