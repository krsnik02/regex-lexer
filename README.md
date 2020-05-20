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
