# tree-grepper

Works like `grep`, but uses `tree-sitter` to search for structure instead of strings.

## Installing

This isn't available packaged anywhere. That's fine, use [`nix`](https://nixos.org/download.html):

`nix-env -if https://git.bytes.zone/brian/tree-grepper/archive/main.tar.gz`

## Usage

Use it like `grep` (or really, more like `ack`/`ag`/`pt`/`rg`.)

```sh
$ tg '(import_clause (import) (upper_case_qid))'
src/Main.elm:3:1:import Browser
src/Main.elm:4:1:import Browser.Navigation
src/main.elm:5:1:import Css
...
```

`tree-grepper` uses [Tree-sitter's s-expressions](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries) to find matches.

The binary name might change in the future if we find a better name. Stay tuned.

## Roadmap

- [ ] be able to do the thing in "Usage" above
- [ ] make capturing sub-matches easy (`@name` in the s-expression syntax)
- [ ] output JSON to make embedding in other tools nicer
- [ ] an option to dump a single file to the s-expression form to make writing matches easier
- [ ] `man` page, nice help output, etc

## License

TODO
