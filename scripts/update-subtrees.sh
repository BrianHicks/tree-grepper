#!/usr/bin/env bash
set -xeuo pipefail

update_subtree() {
  NAME="${1:-}"
  REPO_URL="${2:-}"
  BRANCH="${3:-master}"

  if test -z "$NAME" || test -z "$REPO_URL"; then
    echo "USAGE: update_subtree NAME REPO_URL"
    return 1
  fi

  git subtree pull --squash --prefix "vendor/$NAME" "$REPO_URL" "$BRANCH" --message "update vendored $NAME"
}

update_subtree tree-sitter-cpp https://github.com/tree-sitter/tree-sitter-cpp
update_subtree tree-sitter-elm https://github.com/elm-tooling/tree-sitter-elm main
update_subtree tree-sitter-haskell https://github.com/tree-sitter/tree-sitter-haskell
update_subtree tree-sitter-javascript https://github.com/tree-sitter/tree-sitter-javascript
update_subtree tree-sitter-ruby https://github.com/tree-sitter/tree-sitter-ruby
update_subtree tree-sitter-rust https://github.com/tree-sitter/tree-sitter-rust
update_subtree tree-sitter-typescript https://github.com/tree-sitter/tree-sitter-typescript
