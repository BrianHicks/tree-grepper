# Changelog

## 2.4.1

- Fix a bug in 2.4.0 that disallowed multiple queries in an invocation

## 2.4.0

- New parsed tree view (try `tree-grepper --show-tree languagename path/to/source.ext`) to help writing queries
- Updated dependencies and grammars

## 2.3.0

- PHP support
- Updated dependencies and grammars

## 2.1.0

- Added JSON Lines output (`--format json-lines`)
- C++ support
- Updated dependencies and grammars
  - The bump from tree-sitter 0.20.0 to 0.20.1 removed many previously-matched but unnamed nodes.
    It shouldn't have been possible to write a non-wildcard tree-grepper query to match on these nodes, so this is hypothetically not a breaking change.

## 2.0.x and prior

We didn't keep release notes for these!
See the README and `--help` output at those tags for what was present then.
