#!/usr/bin/env bash
set -xeuo pipefail

git subtree pull --squash --prefix vendor/tree-sitter-elm https://github.com/elm-tooling/tree-sitter-elm main
git subtree pull --squash --prefix vendor/tree-sitter-haskell https://github.com/tree-sitter/tree-sitter-haskell master
git subtree pull --squash --prefix vendor/tree-sitter-javascript https://github.com/tree-sitter/tree-sitter-javascript master
git subtree pull --squash --prefix vendor/tree-sitter-ruby https://github.com/tree-sitter/tree-sitter-ruby master
git subtree pull --squash --prefix vendor/tree-sitter-rust https://github.com/tree-sitter/tree-sitter-rust master
git subtree pull --squash --prefix vendor/tree-sitter-typescript https://github.com/tree-sitter/tree-sitter-typescript master
