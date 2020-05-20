# regex-lexer
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