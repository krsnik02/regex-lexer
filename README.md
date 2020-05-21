# regex-lexer
[<img alt="github" src="https://img.shields.io/badge/github-krsnik02/regex--lexer-8da0cb?style=flat-square&labelColor=555555&logo=github" height="20">](https://github.com/krsnik02/regex-lexer)
[<img alt="crates.io" src="https://img.shields.io/crates/v/regex-lexer.svg?style=flat-square&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/regex-lexer)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-regex--lexer-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/regex-lexer)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/krsnik02/regex-lexer/Rust/master?style=flat-square" height="20">](https://github.com/krsnik02/regex-lexer/actions?query=branch%3Amaster)

A regex-based lexer (tokenizer) in Rust.

## Basic Usage
```rust
enum Token {
    Num(usize),
    // ...
}

let lexer = regex_lexer::LexerBuilder::new()
  .token(r"[0-9]+", |num| Some(Token::Num(num.parse().unwrap())))
  .token(r"\s+", |_| None) // skip whitespace
  // ...
  .build();
  
let tokens = lexer.tokens(/* source */);
```

## License

Licensed under either of

  * Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT License
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusing in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
