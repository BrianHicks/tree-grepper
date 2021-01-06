# tree-grepper

Works like `grep`, but uses `tree-sitter` to search for structure instead of strings.

## Installing

This isn't available packaged anywhere. That's fine, use [`nix`](https://nixos.org/download.html):

`nix-env -if https://git.bytes.zone/brian/tree-grepper/archive/main.tar.gz`

## Usage

Use it like `grep` (or really, more like `ack`/`ag`/`pt`/`rg`.)

```sh
$ tree-grepper '(import_clause (import) (upper_case_qid)@name)'
src/Main.elm:3:1:Browser
src/Main.elm:4:1:Browser.Navigation
src/main.elm:5:1:Css
...
```

`tree-grepper` uses [Tree-sitter's s-expressions](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries) to find matches.

The binary name might change in the future if we find a better/shorter name. Stay tuned.

## Some Terrible Benchmarks

On the first possible working version of `tree-grepper`:

| Command                                 | Mean Time (Hyperfine) |
|-----------------------------------------|----------------------:|
| `tree-grepper '(import_clause)@import'` | 17.2ms                |
| `rg -t elm '^import'`                   | 10.3ms                |
| `grep -rE '^import'`                    | 71.0ms                |

So this is on `rg`'s level of quickness (which makes sense, as this tool uses their tree walking/gitignoring library.)
This tool may get slower as we add features, or faster as I learn more about how to write good Rust.

## Roadmap

- [x] be able to do the thing in "Usage" above
- [x] output JSON to make embedding in other tools nicer
- [x] make capturing sub-matches easy (`@name` in the s-expression syntax)
- [ ] make this tool work on a bunch of languages, not just Elm (which I'm starting with to scratch an itch.)
- [ ] get conditionals working (`#eq?`, `#match?` from the tree-sitter docs don't work yet)
- [ ] add tests
- [ ] an option to dump a single file to the s-expression form to make writing matches easier
- [ ] `man` page, nice help output, etc
- [ ] produce a query from a language's syntax instead of having to write s-expressions directly
- [ ] real/reproducible benchmarks

## License

TODO
