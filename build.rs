use std::path::PathBuf;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // c
    let c_dir: PathBuf = ["vendor", "tree-sitter-c", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-c/src/parser.c");
    cc::Build::new()
        .include(&c_dir)
        .warnings(false)
        .file(c_dir.join("parser.c"))
        .compile("tree-sitter-c");

    // cpp
    let cpp_dir: PathBuf = ["vendor", "tree-sitter-cpp", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-cpp/src/parser.c");
    cc::Build::new()
        .include(&cpp_dir)
        .warnings(false)
        .file(cpp_dir.join("parser.c"))
        .compile("tree-sitter-cpp");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-cpp/src/scanner.c");
    cc::Build::new()
        .include(&cpp_dir)
        .warnings(false)
        .file(cpp_dir.join("scanner.c"))
        .compile("tree_sitter_cpp_scanner");

    // elixir
    let elixir_dir: PathBuf = ["vendor", "tree-sitter-elixir", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elixir/src/parser.c");
    cc::Build::new()
        .include(&elixir_dir)
        .warnings(false)
        .file(elixir_dir.join("parser.c"))
        .compile("tree-sitter-elixir");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elixir/src/scanner.c");
    cc::Build::new()
        .include(&elixir_dir)
        .warnings(false)
        .file(elixir_dir.join("scanner.c"))
        .compile("tree_sitter_elixir_scanner");

    // elm
    let elm_dir: PathBuf = ["vendor", "tree-sitter-elm", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/parser.c");
    cc::Build::new()
        .include(&elm_dir)
        .warnings(false)
        .file(elm_dir.join("parser.c"))
        .compile("tree-sitter-elm");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-elm/src/scanner.c");
    cc::Build::new()
        .include(&elm_dir)
        .warnings(false)
        .file(elm_dir.join("scanner.c"))
        .compile("tree_sitter_elm_scanner");

    // go
    let go_dir: PathBuf = ["vendor", "tree-sitter-go", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-go/src/parser.c");
    cc::Build::new()
        .include(&go_dir)
        .warnings(false)
        .file(go_dir.join("parser.c"))
        .compile("tree-sitter-go");

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
        .file(haskell_dir.join("scanner.c"))
        .compile("tree_sitter_haskell_scanner");

    // java
    let java_dir: PathBuf = ["vendor", "tree-sitter-java", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-java/src/parser.c");
    cc::Build::new()
        .include(&java_dir)
        .warnings(false)
        .file(java_dir.join("parser.c"))
        .compile("tree-sitter-java");

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

    // markdown
    let markdown_dir: PathBuf = ["vendor", "tree-sitter-markdown", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-markdown/src/parser.c");
    cc::Build::new()
        .include(&markdown_dir)
        .warnings(false)
        .file(markdown_dir.join("parser.c"))
        .compile("tree-sitter-markdown");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-markdown/src/scanner.cc");
    cc::Build::new()
        .include(&markdown_dir)
        .cpp(true)
        .warnings(false)
        .file(markdown_dir.join("scanner.cc"))
        .compile("tree_sitter_markdown_scanner");

    // nix
    let nix_dir: PathBuf = ["vendor", "tree-sitter-nix", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-nix/src/parser.c");
    cc::Build::new()
        .include(&nix_dir)
        .warnings(false)
        .file(nix_dir.join("parser.c"))
        .compile("tree-sitter-nix");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-nix/src/scanner.c");
    cc::Build::new()
        .include(&nix_dir)
        .warnings(false)
        .file(nix_dir.join("scanner.c"))
        .compile("tree_sitter_nix_scanner");

    // php
    let php_dir: PathBuf = ["vendor", "tree-sitter-php", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-php/src/parser.c");
    cc::Build::new()
        .include(&php_dir)
        .warnings(false)
        .file(php_dir.join("parser.c"))
        .compile("tree-sitter-php");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-php/src/scanner.c");
    cc::Build::new()
        .include(&php_dir)
        .warnings(false)
        .file(php_dir.join("scanner.c"))
        .compile("tree_sitter_php_scanner");

    // python
    let python_dir: PathBuf = ["vendor", "tree-sitter-python", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-python/src/parser.c");
    cc::Build::new()
        .include(&python_dir)
        .warnings(false)
        .file(python_dir.join("parser.c"))
        .compile("tree-sitter-python");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-python/src/scanner.c");
    cc::Build::new()
        .include(&python_dir)
        .warnings(false)
        .file(python_dir.join("scanner.c"))
        .compile("tree_sitter_python_scanner");

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
}
