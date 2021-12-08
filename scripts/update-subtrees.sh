#!/usr/bin/env bash
set -xeuo pipefail

git subtree pull --squash --prefix vendor/tree-sitter-cpp https://github.com/tree-sitter/tree-sitter-cpp master --message "update vendored tree-sitter-cpp"
git subtree pull --squash --prefix vendor/tree-sitter-elm https://github.com/elm-tooling/tree-sitter-elm main --message "update vendored tree-sitter-elm"
git subtree pull --squash --prefix vendor/tree-sitter-haskell https://github.com/tree-sitter/tree-sitter-haskell master --message "update vendored tree-sitter-haskell"
git subtree pull --squash --prefix vendor/tree-sitter-javascript https://github.com/tree-sitter/tree-sitter-javascript master --message "update vendored tree-sitter-javascript"
git subtree pull --squash --prefix vendor/tree-sitter-ruby https://github.com/tree-sitter/tree-sitter-ruby master --message "update vendored tree-sitter-ruby"
git subtree pull --squash --prefix vendor/tree-sitter-rust https://github.com/tree-sitter/tree-sitter-rust master --message "update vendored tree-sitter-rust"
git subtree pull --squash --prefix vendor/tree-sitter-typescript https://github.com/tree-sitter/tree-sitter-typescript master --message "update vendored tree-sitter-typescript"
