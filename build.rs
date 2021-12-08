use std::path::PathBuf;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // elm
    let elm_dir: PathBuf = ["vendor", "tree-sitter-elm", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/parser.c");
    cc::Build::new()
        .include(&elm_dir)
        .warnings(false)
        .file(elm_dir.join("parser.c"))
        .compile("tree-sitter-elm");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/scanner.cc");
    cc::Build::new()
        .include(&elm_dir)
        .cpp(true)
        .warnings(false)
        .file(elm_dir.join("scanner.cc"))
        .compile("tree_sitter_elm_scanner");

    // ruby
    let ruby_dir: PathBuf = ["vendor", "tree-sitter-ruby", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-ruby/src/parser.c");
    cc::Build::new()
        .include(&ruby_dir)
        .warnings(false)
        .file(ruby_dir.join("parser.c"))
        .compile("tree-sitter-ruby");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-ruby/src/scanner.cc");
    cc::Build::new()
        .include(&ruby_dir)
        .cpp(true)
        .warnings(false)
        .file(ruby_dir.join("scanner.cc"))
        .compile("tree_sitter_ruby_scanner");

    // javascript
    let javascript_dir: PathBuf = ["vendor", "tree-sitter-javascript", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-javascript/src/parser.c");
    cc::Build::new()
        .include(&javascript_dir)
        .warnings(false)
        .file(javascript_dir.join("parser.c"))
        .compile("tree-sitter-javascript");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-javascript/src/scanner.c");
    cc::Build::new()
        .include(&javascript_dir)
        .warnings(false)
        .file(javascript_dir.join("scanner.c"))
        .compile("tree_sitter_javascript_scanner");

    // haskell
    let haskell_dir: PathBuf = ["vendor", "tree-sitter-haskell", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-haskell/src/parser.c");
    cc::Build::new()
        .include(&haskell_dir)
        .warnings(false)
        .file(haskell_dir.join("parser.c"))
        .compile("tree-sitter-haskell");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-haskell/src/scanner.cc");
    cc::Build::new()
        .include(&haskell_dir)
        .warnings(false)
        .cpp(true)
        .file(haskell_dir.join("scanner.cc"))
        .compile("tree_sitter_haskell_scanner");

    // rust
    let rust_dir: PathBuf = ["vendor", "tree-sitter-rust", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-rust/src/parser.c");
    cc::Build::new()
        .include(&rust_dir)
        .warnings(false)
        .file(rust_dir.join("parser.c"))
        .compile("tree-sitter-rust");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-rust/src/scanner.c");
    cc::Build::new()
        .include(&rust_dir)
        .warnings(false)
        .file(rust_dir.join("scanner.c"))
        .compile("tree_sitter_rust_scanner");

    // typescript
    let typescript_dir: PathBuf = ["vendor", "tree-sitter-typescript", "typescript", "src"]
        .iter()
        .collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-typescript/typescript/src/parser.c");
    cc::Build::new()
        .include(&typescript_dir)
        .warnings(false)
        .file(typescript_dir.join("parser.c"))
        .compile("tree-sitter-typescript");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-typescript/typescript/src/scanner.c");
    cc::Build::new()
        .include(&typescript_dir)
        .warnings(false)
        .file(typescript_dir.join("scanner.c"))
        .compile("tree_sitter_typescript_scanner");
    // cpp
    let cpp_dir: PathBuf = ["vendor", "tree-sitter-cpp", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-cpp/src/parser.c");
    cc::Build::new()
        .include(&cpp_dir)
        .warnings(false)
        .file(cpp_dir.join("parser.c"))
        .compile("tree-sitter-cpp");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-cpp/src/scanner.cc");
    cc::Build::new()
        .include(&cpp_dir)
        .cpp(true)
        .warnings(false)
        .file(cpp_dir.join("scanner.cc"))
        .compile("tree_sitter_cpp_scanner");
}
