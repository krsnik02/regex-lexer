#![doc(html_root_url = "https://docs.rs/regex-lexer/0.2.1/regex-lexer")]
//! A regex-based lexer (tokenizer).
//!
//! ```
//! use regex_lexer::{LexerBuilder, Token};
//!
//! #[derive(Debug, PartialEq, Eq, Clone, Copy)]
//! enum Tok {
//!     Num,
//!     Add,
//!     Sub,
//!     Mul,
//!     Div,
//!     Open,
//!     Close,
//! }
//!
//! let lexer = LexerBuilder::new()
//!     .token(r"[0-9]+", Tok::Num)
//!     .token(r"\+", Tok::Add)
//!     .token(r"-", Tok::Sub)
//!     .token(r"\*", Tok::Mul)
//!     .token(r"/", Tok::Div)
//!     .token(r"\(", Tok::Open)
//!     .token(r"\)", Tok::Close)
//!     .ignore(r"\s+")
//!     .build()?;
//!
//! let source = "(1 + 2) * 3";
//! assert_eq!(
//!     lexer.tokens(source).collect::<Vec<_>>(),
//!     vec![
//!         Token { kind: Tok::Open, span: 0..1, text: "(" },
//!         Token { kind: Tok::Num, span: 1..2, text: "1" },
//!         Token { kind: Tok::Add, span: 3..4, text: "+" },
//!         Token { kind: Tok::Num, span: 5..6, text: "2" },
//!         Token { kind: Tok::Close, span: 6..7, text: ")" },
//!         Token { kind: Tok::Mul, span: 8..9, text: "*" },
//!         Token { kind: Tok::Num, span: 10..11, text: "3" },
//!     ],
//! );
//! # Ok::<(), regex_lexer::Error>(())
//! ```

use std::ops::Range;

pub use regex::Error;
use regex::{Regex, RegexSet};

/// A token returned by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'t, K> {
    pub kind: K,
    pub span: Range<usize>,
    pub text: &'t str,
}

/// Builder struct for [Lexer](struct.Lexer.html).
pub struct LexerBuilder<'r, K> {
    regexes: Vec<&'r str>,
    kinds: Vec<Option<K>>,
}

impl<'r, K> Default for LexerBuilder<'r, K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'r, K> LexerBuilder<'r, K> {
    /// Create a new [LexerBuilder](struct.LexerBuilder.html).
    pub fn new() -> Self {
        LexerBuilder {
            regexes: Vec::new(),
            kinds: Vec::new(),
        }
    }

    /// Add a new token that matches the regular expression `re`.
    /// This uses the same syntax as the [regex](http://docs.rs/regex/1/regex) crate.
    ///
    /// The passed regex must NOT be able to match a zero-length string. If it does
    /// then the following call to [LexerBuilder::build](struct.LexerBuilder.html#method.build)
    /// will panic.
    ///
    /// If the regex matches, it will return a token of kind `kind`.
    /// ```
    /// use regex_lexer::{LexerBuilder, Token};
    ///
    /// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// enum Tok {
    ///     Num,
    ///     // ...
    /// }
    ///
    /// let lexer = LexerBuilder::new()
    ///     .token(r"[0-9]+", Tok::Num)
    ///     .ignore(r"\s+") // skip whitespace
    ///     // ...
    ///     .build()?;
    ///
    /// assert_eq!(
    ///     lexer.tokens("1 2 3").collect::<Vec<_>>(),
    ///     vec![
    ///         Token { kind: Tok::Num, span: 0..1, text: "1" },
    ///         Token { kind: Tok::Num, span: 2..3, text: "2" },
    ///         Token { kind: Tok::Num, span: 4..5, text: "3" },
    ///     ],
    /// );
    /// # Ok::<(), regex::Error>(())
    /// ```
    ///
    /// If multiple regexes all match, then whichever is defined last
    /// will be given priority.
    /// ```
    /// use regex_lexer::{LexerBuilder, Token};
    ///
    /// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// enum Tok {
    ///     Ident,
    ///     Let,
    ///     // ...
    /// }
    ///
    /// let lexer = LexerBuilder::new()
    ///     .token(r"[a-zA-Z_][a-zA-Z0-9_]*", Tok::Ident)
    ///     .token(r"let\b", Tok::Let)
    ///     // ...
    ///     .ignore(r"\s+")
    ///     .build()?;
    ///
    /// assert_eq!(
    ///     lexer.tokens("let lettuce").collect::<Vec<_>>(),
    ///     vec![
    ///         Token { kind: Tok::Let, span: 0..3, text: "let" },
    ///         Token { kind: Tok::Ident, span: 4..11, text: "lettuce" },
    ///     ],
    /// );
    /// # Ok::<(), regex::Error>(())
    /// ```
    pub fn token(mut self, re: &'r str, kind: K) -> Self {
        self.add_regex(re, Some(kind));
        self
    }

    /// Add a new regex which if matched will ignore the matched text.
    ///
    /// The passed regex must NOT be able to match a zero-length string. If it does
    /// then the following call to [LexerBuilder::build](struct.LexerBuilder.html#method.build)
    /// will panic.
    pub fn ignore(mut self, re: &'r str) -> Self {
        self.add_regex(re, None);
        self
    }

    /// Inserts a regex into the lexer matching the specified kind.
    fn add_regex(&mut self, re: &'r str, kind: Option<K>) {
        self.regexes.push(re);
        self.kinds.push(kind);
    }

    /// Construct a [Lexer](struct.Lexer.html) which matches these tokens.
    ///
    /// ## Errors
    ///
    /// If a regex cannot be compiled, a [Error](https://docs.rs/regex/1/regex/enum.Error.html) is returned.
    ///
    /// ## Panics
    ///
    /// Panics if any of the passed regexes would match a zero-length string.
    pub fn build(self) -> Result<Lexer<K>, Error> {
        let regexes = self.regexes.into_iter().map(|r| format!("^(?:{})", r));
        let regex_set = RegexSet::new(regexes)?;

        let mut regexes = Vec::new();
        for pattern in regex_set.patterns() {
            let regex = Regex::new(pattern)?;
            if regex.is_match("") {
                panic!("Regex {regex:?} can match a zero-length string");
            }
            regexes.push(regex);
        }

        Ok(Lexer {
            kinds: self.kinds,
            regexes,
            regex_set,
        })
    }
}

/// A regex-based lexer.
///
/// ```
/// use regex_lexer::{LexerBuilder, Token};
///
/// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// enum Tok {
///     Ident,
///     // ...
/// }
///
/// let lexer = LexerBuilder::new()
///     .token(r"\p{XID_Start}\p{XID_Continue}*", Tok::Ident)
///     .ignore(r"\s+") // skip whitespace
///     // ...
///     .build()?;
///
/// let tokens = lexer.tokens("these are some identifiers");
///
/// # assert_eq!(
/// #    tokens.collect::<Vec<_>>(),
/// #    vec![
/// #        Token { kind: Tok::Ident, span: 0..5, text: "these" },
/// #        Token { kind: Tok::Ident, span: 6..9, text: "are" },
/// #        Token { kind: Tok::Ident, span: 10..14, text: "some" },
/// #        Token { kind: Tok::Ident, span: 15..26, text: "identifiers" },
/// #    ],
/// # );
/// # Ok::<(), regex::Error>(())
/// ```
#[derive(Debug)]
pub struct Lexer<K> {
    kinds: Vec<Option<K>>,
    regexes: Vec<Regex>,
    regex_set: RegexSet,
}

impl<K> Lexer<K> {
    /// Create a [LexerBuilder](struct.LexerBuilder.html). This is the same as [LexerBuilder::new](struct.LexerBuilder.html#method.new).
    pub fn builder<'r>() -> LexerBuilder<'r, K> {
        LexerBuilder::new()
    }

    /// Return an iterator over all matched tokens.
    pub fn tokens<'l, 't>(&'l self, source: &'t str) -> Tokens<'l, 't, K> {
        Tokens {
            lexer: self,
            source,
            position: 0,
        }
    }
}

/// The type returned by [Lexer::tokens](struct.Lexer.html#method.tokens).
#[derive(Debug)]
pub struct Tokens<'l, 't, K> {
    lexer: &'l Lexer<K>,
    source: &'t str,
    position: usize,
}

impl<'l, 't, K: Copy> Iterator for Tokens<'l, 't, K> {
    type Item = Token<'t, K>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.position == self.source.len() {
                return None;
            }

            let string = &self.source[self.position..];
            let match_set = self.lexer.regex_set.matches(string);
            let (len, i) = match_set
                .into_iter()
                .map(|i: usize| {
                    let m = self.lexer.regexes[i].find(string).unwrap();
                    assert!(m.start() == 0);
                    (m.end(), i)
                })
                //.max_by_key(|(len, _)| *len)
                .next_back()
                .unwrap();

            let span = self.position..self.position + len;
            let text = &self.source[span.clone()];
            self.position += len;
            match self.lexer.kinds[i] {
                Some(kind) => return Some(Token { kind, span, text }),
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // issue #4
    #[test]
    #[should_panic]
    fn zero_length_matches() {
        #[derive(Debug, Copy, Clone)]
        enum Tok {
            Num,
        }

        let _lexer = LexerBuilder::new()
            .token(r"[0-9]*", Tok::Num)
            .ignore(r"\s+")
            .build()
            .unwrap();
    }

    // issue #5
    #[test]
    fn top_level_alteration() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Tok {
            Ws,
            Word,
        }

        let lexer = LexerBuilder::new()
            .token(r"\s+", Tok::Ws)
            .token(r"foo|bar", Tok::Word)
            .build()
            .unwrap();

        let tokens: Vec<_> = lexer.tokens("foo bar").collect();
        assert_eq!(
            tokens,
            [
                Token {
                    kind: Tok::Word,
                    span: 0..3,
                    text: "foo"
                },
                Token {
                    kind: Tok::Ws,
                    span: 3..4,
                    text: " "
                },
                Token {
                    kind: Tok::Word,
                    span: 4..7,
                    text: "bar"
                },
            ]
        );
    }
}
