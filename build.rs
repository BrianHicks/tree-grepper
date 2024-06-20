extern crate git2;

use git2::Repository;
use std::path::Path;
use std::path::PathBuf;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let repos = vec![
        ("tree-sitter-c", "https://github.com/tree-sitter/tree-sitter-c"),
        ("tree-sitter-cpp", "https://github.com/tree-sitter/tree-sitter-cpp"),
        ("tree-sitter-cuda", "https://github.com/theHamsta/tree-sitter-cuda"),
        ("tree-sitter-elixir", "https://github.com/elixir-lang/tree-sitter-elixir"),
        ("tree-sitter-elm", "https://github.com/elm-tooling/tree-sitter-elm"),
        ("tree-sitter-go", "https://github.com/tree-sitter/tree-sitter-go"),
        ("tree-sitter-haskell", "https://github.com/tree-sitter/tree-sitter-haskell"),
        ("tree-sitter-java", "https://github.com/tree-sitter/tree-sitter-java"),
        ("tree-sitter-javascript", "https://github.com/tree-sitter/tree-sitter-javascript"),
        ("tree-sitter-markdown", "https://github.com/tree-sitter-grammars/tree-sitter-markdown"),
        ("tree-sitter-nix", "https://github.com/cstrahan/tree-sitter-nix"),
        ("tree-sitter-php", "https://github.com/tree-sitter/tree-sitter-php"),
        ("tree-sitter-python", "https://github.com/tree-sitter/tree-sitter-python"),
        ("tree-sitter-ruby", "https://github.com/tree-sitter/tree-sitter-ruby"),
        ("tree-sitter-rust", "https://github.com/tree-sitter/tree-sitter-rust"),
        ("tree-sitter-scss", "https://github.com/serenadeai/tree-sitter-scss"),
        ("tree-sitter-typescript", "https://github.com/tree-sitter/tree-sitter-typescript"),
    ];


    let vendor_dir = Path::new("vendor");

    if !vendor_dir.exists() {
        std::fs::create_dir(vendor_dir).unwrap();
    }

    for (name, url) in repos {
        let repo_path = vendor_dir.join(name);
        if !repo_path.exists() {
            println!("Cloning {} into {:?}", url, repo_path);
            match Repository::clone(url, &repo_path) {
                Ok(_) => println!("Successfully cloned {}", url),
                Err(e) => eprintln!("Failed to clone {}: {}", url, e),
            }
        } else {
            println!("{} already exists, skipping", repo_path.display());
        }
    }

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

    // cuda
    let cuda_dir: PathBuf = ["vendor", "tree-sitter-cuda", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-cuda/src/parser.c");
    cc::Build::new()
        .include(&cuda_dir)
        .warnings(false)
        .file(cuda_dir.join("parser.c"))
        .compile("tree-sitter-cuda");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-cuda/src/scanner.c");
    cc::Build::new()
        .include(&cuda_dir)
        .warnings(false)
        .file(cuda_dir.join("scanner.c"))
        .compile("tree_sitter_cuda_scanner");

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

    println!("cargo:rerun-if-changed=vendor/tree-sitter-haskell/src/scanner.c");
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
    let markdown_dir: PathBuf = [
        "vendor",
        "tree-sitter-markdown",
        "tree-sitter-markdown",
        "src",
    ]
    .iter()
    .collect();

    println!(
        "cargo:rerun-if-changed=vendor/tree-sitter-markdown/tree-sitter-markdown/src/parser.c"
    );
    cc::Build::new()
        .include(&markdown_dir)
        .warnings(false)
        .file(markdown_dir.join("parser.c"))
        .compile("tree-sitter-markdown");

    println!(
        "cargo:rerun-if-changed=vendor/tree-sitter-markdown/tree-sitter-markdown/src/scanner.c"
    );
    cc::Build::new()
        .include(&markdown_dir)
        .warnings(false)
        .file(markdown_dir.join("scanner.c"))
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
    let php_dir: PathBuf = ["vendor", "tree-sitter-php", "php", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-php/php/src/parser.c");
    cc::Build::new()
        .include(&php_dir)
        .warnings(false)
        .file(php_dir.join("parser.c"))
        .compile("tree-sitter-php");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-php/php/src/scanner.c");
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

    println!("cargo:rerun-if-changed=vendor/tree-sitter-ruby/src/scanner.c");
    cc::Build::new()
        .include(&ruby_dir)
        .warnings(false)
        .file(ruby_dir.join("scanner.c"))
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

    // scss
    let scss_dir: PathBuf = ["vendor", "tree-sitter-scss", "src"].iter().collect();

    println!("cargo:rerun-if-changed=vendor/tree-sitter-scss/src/parser.c");
    cc::Build::new()
        .include(&scss_dir)
        .warnings(false)
        .file(scss_dir.join("parser.c"))
        .compile("tree-sitter-scss");

    println!("cargo:rerun-if-changed=vendor/tree-sitter-scss/src/scanner.c");
    cc::Build::new()
        .include(&scss_dir)
        .warnings(false)
        .file(scss_dir.join("scanner.c"))
        .compile("tree_sitter_scss_scanner");

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
