# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2026-08-04

### Changed
- Fixed `Tokens::match` to return `None` instead of panicing when no regexes match
  the input.
- Forbid all regexes that can match a zero-length string. These will now panic in 
  `TokenBuilder::build`.
- Fixed behavior of top-level alternation in regexes. Previously later options could
  match mid-string, resulting in a panic.

## [0.2.1] - 2026-01-25

### Changed
- Bump to Rust edition 2025.

## [0.2.0] - 2022-08-14

### Added
- `Token` type including spans.

### Changed
- Split `token` function into one to create a token and another to ignore the match.

### Removed
- Action functions have been removed. The `token` function now takes a user-defined type 
  specifying the kind of the token.

## [0.1.1] - 2022-08-14

### Changed
- Action functions are now taken by `FnMut` rather than `Fn`.

## [0.1.0] - 2020-05-20

### Added
- Basic lexer functionality.

[0.2.2]: https://github.com/krsnik02/regex-lexer/releases/tag/v0.2.2
[0.2.1]: https://github.com/krsnik02/regex-lexer/releases/tag/v0.2.1
[0.2.0]: https://github.com/krsnik02/regex-lexer/releases/tag/v0.2.0
[0.1.1]: https://github.com/krsnik02/regex-lexer/releases/tag/v0.1.1
[0.1.0]: https://github.com/krsnik02/regex-lexer/releases/tag/v0.1.0
