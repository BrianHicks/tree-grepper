#!/usr/bin/env bash
set -euo pipefail

git subtree pull --squash --prefix vendor/tree-sitter-elm https://github.com/Razzeee/tree-sitter-elm master
