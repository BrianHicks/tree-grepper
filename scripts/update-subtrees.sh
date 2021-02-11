#!/usr/bin/env bash
set -euo pipefail

git subtree pull --squash --prefix vendor/tree-sitter-elm https://github.com/Razzeee/tree-sitter-elm master
git subtree pull --squash --prefix vendor/tree-sitter-ruby https://github.com/tree-sitter/tree-sitter-ruby master
git subtree pull --squash --prefix vendor/tree-sitter-javascript https://github.com/tree-sitter/tree-sitter-javascript master
