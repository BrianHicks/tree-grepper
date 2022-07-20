use std::path::PathBuf;

fn build_langs(langs: &[(&str, Option<&str>, &[(&str, bool)])]) {
    for (lang, subdir, files) in langs {
        let mut dir = PathBuf::new();
        let repo_name = format!("tree-sitter-{}", lang);
        dir.push("vendor");
        dir.push(&repo_name);
        if let Some(subdir) = subdir {
            dir.push(subdir);
        }
        dir.push("src");
        for (file, cpp) in *files {
            let loc = dir.join(file);
            println!("cargo:rerun-if-changed={}", loc.display());
            cc::Build::new()
                .include(&dir)
                .warnings(false)
                .cpp(*cpp)
                .file(loc)
                .compile(&format!("{}_{}", repo_name, file));
        }
    }
}

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_langs(&[
        ("cpp", None, &[("parser.c", false), ("scanner.cc", true)]),
        ("elixir", None, &[("parser.c", false), ("scanner.cc", true)]),
        ("elm", None, &[("parser.c", false), ("scanner.cc", true)]),
        ("haskell", None, &[("parser.c", false), ("scanner.c", false)]),
        ("javascript", None, &[("parser.c", false), ("scanner.c", false)]),
        ("markdown", None, &[("parser.c", false), ("scanner.cc", true)]),
        ("nix", None, &[("parser.c", false), ("scanner.c", false)]),
        ("php", None, &[("parser.c", false), ("scanner.cc", true)]),
        ("ruby", None, &[("parser.c", false), ("scanner.cc", true)]),
        ("python", None, &[("parser.c", false), ("scanner.cc", false)]),
        ("rust", None, &[("parser.c", false), ("scanner.c", false)]),
        ("typescript", Some("typescript"), &[("parser.c", false), ("scanner.c", false)]),
    ]);
}
