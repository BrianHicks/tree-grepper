use std::path::PathBuf;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // elm
    let elm_dir: PathBuf = ["vendor", "tree-sitter-elm", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/parser.c");
    cc::Build::new()
        .include(&elm_dir)
        .file(elm_dir.join("parser.c"))
        .compile("tree-sitter-elm");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/scanner.cc");
    cc::Build::new()
        .include(&elm_dir)
        .cpp(true)
        .file(elm_dir.join("scanner.cc"))
        .compile("tree_sitter_elm_scanner");

    // ruby
    let ruby_dir: PathBuf = ["vendor", "tree-sitter-ruby", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-ruby/src/parser.c");
    cc::Build::new()
        .include(&ruby_dir)
        .file(ruby_dir.join("parser.c"))
        .compile("tree-sitter-ruby");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-ruby/src/scanner.cc");
    cc::Build::new()
        .include(&ruby_dir)
        .cpp(true)
        .file(ruby_dir.join("scanner.cc"))
        .compile("tree_sitter_ruby_scanner");
}
