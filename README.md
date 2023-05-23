# tree-grepper

Works like `grep`, but uses `tree-sitter` to search for structure instead of strings.
[Here's a longer introduction to the tool as a blog post](https://bytes.zone/posts/tree-grepper/).

## Installing

Use [`nix`](https://nixos.org/download.html) to install:

1. if you have `cachix` installed, `cachix use tree-grepper`.
2. `nix-env -if https://github.com/BrianHicks/tree-grepper/archive/refs/heads/main.tar.gz`

## Usage

Use it like `grep` (or really, more like `ack`/`ag`/`pt`/`rg`.)

```console
$ tree-grepper -q javascript '(call_expression)'
./tests/cmd/hello-world.js:1:1:query:console.log("Hello, World!")

```

By default, `tree-grepper` will output one match per (newline-delimited) line.
The columns here are filename, row, column, match name, and match text.

Note, however, that if your query includes a match with newlines in the text they will be included in the output!
If this causes problems for your use case, try asking for JSON output (`-f json`) instead.

`tree-grepper` uses Tree-sitter's s-expressions to find matches.
See [the tree-sitter docs on queries](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries) for what all you can do there.

We add one important thing on top of the standard query stuff (including `#eq?` and `#match?`): if you name a pattern starting with an underscore, it will not be returned in the output.
This is primarily useful for filtering out matches you don't really care about.
For example, to match JavaScript calls to `require` but not other functions, you could do this:

```scheme
(call_expression (identifier)@_fn (arguments . (string)@import .) (#eq? @_fn require))
```

In addition to text output, we support JSON output for scripting: just  specify `-f json`.
You also get more info (the match's end location and node kind) by asking for JSON output.

### Tree View

You can discover the node names your language uses by using `--show-tree languagename path/to/file`.
When you do this, `tree-grepper` will parse the file and print out an indented tree view.

For a file like this:

```javascript
console.log("Hello, World!");
```

You'll get a tree like this:

```console
$ tree-grepper --show-tree javascript tests/cmd/hello-world.js
program 1:1
  expression_statement 1:1
    call_expression 1:1
      member_expression 1:1
        identifier 1:1: console
        . 1:8: .
        property_identifier 1:9: log
      arguments 1:12
        ( 1:12: (
        string 1:13
          " 1:13: "
          string_fragment 1:14: Hello, World!
          " 1:27: "
        ) 1:28: )
    ; 1:29: ;

```

Each line takes the format `{node name} {location} {source, if present}`.
Source is only shown for the leaf-most nodes on the tree to avoid printing a huge block.
However, tree-grepper can extract text from any of these nodes.

You can use the node names in queries.
For example:

- `tree-grepper -q elm (exposed_value)` would have matches on `average` and `percentOf`.
- `tree-grepper -q elm (module_declaration)` would match on the whole declaration, `module Math exposing (average, percentOf)`

## Supported Languages

You can find all the languages in your installed version of `tree-grepper` by running `tree-grepper --languages`.
The output is the literal language strings you can use in queries or `--show-tree`.
For example, here's the list for the current version of `tree-grepper`:

```console
$ tree-grepper --languages
c
cpp
elixir
elm
go
haskell
java
javascript
markdown
nix
php
python
ruby
rust
typescript

````

Don't see your favorite?
We're open to PRs for adding whatever language you'd like!

For development, there's a nix-shell setup that'll get you everything you need.
Set up [nix](https://nixos.org/download.html) (just Nix, not NixOS) and then run `nix-shell` in the root of this repository.

After that, you just need to add a tree-sitter grammar to the project.
[The tree-sitter project keeps an up-to-date list](https://tree-sitter.github.io/tree-sitter/), so you may not even need to write your own!

Note: when you're adding grammars, please keep things in alphabetical order.

1. Add your grammar as an input in `flakes.nix`, following the template of the ones already there.
   You'll need to add an entry in `inputs` and another in the `updateVendor` script.
2. Run `direnv reload` to make sure you have the latest changes, then `update-vendor` to get your grammar in the right place.
   Make sure the repo content under `vendor/YOUR-GRAMMAR` looks how you expect.
3. Set up compilation in [`build.rs`](./build.rs) by following the pattern there.
4. Set up a new target in [`src/language.rs`](./src/language.rs) by following the patterns there.
5. Add the language to the list of supported languages in this readme.

## License

See [LICENSE](./LICENSE) in the source.
