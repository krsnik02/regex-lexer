#![doc(html_root_url = "https://docs.rs/regex-lexer/0.1.0")]
//! A regex-based lexer (tokenizer).
//!
//! ```
//! use regex_lexer::LexerBuilder;
//!
//! #[derive(Debug, PartialEq, Eq)]
//! enum Token {
//!     Num(u32),
//!     Add,
//!     Sub,
//!     Mul,
//!     Div,
//!     Open,
//!     Close,
//! }
//!
//! let lexer = LexerBuilder::new()
//!     .token(r"[0-9]+", |tok| Some(Token::Num(tok.parse().unwrap())))
//!     .token(r"\+", |_| Some(Token::Add))
//!     .token(r"-", |_| Some(Token::Sub))
//!     .token(r"\*", |_| Some(Token::Mul))
//!     .token(r"/", |_| Some(Token::Div))
//!     .token(r"\(", |_| Some(Token::Open))
//!     .token(r"\)", |_| Some(Token::Close))
//!     .token(r"\s+", |_| None) // skip whitespace
//!     .build()?;
//!
//! let source = "(1 + 2) * 3";
//! assert_eq!(
//!     lexer.tokens(source).collect::<Vec<_>>(),
//!     vec![
//!         Token::Open, Token::Num(1), Token::Add, Token::Num(2), Token::Close,
//!         Token::Mul, Token::Num(3)
//!     ],
//! );
//! # Ok::<(), regex::Error>(())
//! ```

use regex::{Regex, RegexSet};

/// Builder struct for [Lexer](struct.Lexer.html).
pub struct LexerBuilder<'r, 't, T: 't> {
    regexes: Vec<&'r str>,
    fns: Vec<Box<dyn Fn(&'t str) -> Option<T>>>,
}

impl<'r, 't, T: 't> std::fmt::Debug for LexerBuilder<'r, 't, T> {
    /// Shows the matched regexes
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LexerBuilder")
            .field("regexes", &self.regexes)
            .finish() // todo: finish_non_exhaustive
    }
}

impl<'r, 't, T: 't> Default for LexerBuilder<'r, 't, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'r, 't, T: 't> LexerBuilder<'r, 't, T> {
    /// Create a new [LexerBuilder](struct.LexerBuilder.html).
    pub fn new() -> Self {
        LexerBuilder {
            regexes: Vec::new(),
            fns: Vec::new(),
        }
    }

    /// Add a new token that matches the regular expression `re`.
    /// This uses the same syntax as the [regex](http://docs.rs/regex) crate.
    ///
    /// If `re` gives the longest match, then `f` is called on the matched string.
    /// * If `f` returns `Some(tok)`, emit the token `tok`.
    /// * Otherwise, skip this token and emit nothing.
    /// ```
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum Token {
    ///     Num(usize),
    ///     // ...
    /// }
    ///
    /// let lexer = regex_lexer::LexerBuilder::new()
    ///     .token(r"[0-9]*", |num| Some(Token::Num(num.parse().unwrap())))
    ///     .token(r"\s+", |_| None) // skip whitespace
    ///     // ...
    ///     .build()?;
    ///
    /// assert_eq!(
    ///     lexer.tokens("1 2 3").collect::<Vec<_>>(),
    ///     vec![Token::Num(1), Token::Num(2), Token::Num(3)],
    /// );
    /// # Ok::<(), regex::Error>(())
    /// ```
    ///
    /// If multiple regexes all have the same longest match, then whichever is defined last
    /// is given priority.
    /// ```
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum Token<'t> {
    ///     Ident(&'t str),
    ///     Then,
    ///     // ...
    /// }
    ///
    /// let lexer = regex_lexer::LexerBuilder::new()
    ///     .token(r"[a-zA-Z_][a-zA-Z0-9_]*", |id| Some(Token::Ident(id)))
    ///     .token(r"then", |_| Some(Token::Then))
    ///     // ...
    ///     .build()?;
    ///
    /// assert_eq!(lexer.tokens("then").next(), Some(Token::Then));
    /// assert_eq!(lexer.tokens("then_perish").next(), Some(Token::Ident("then_perish")));
    /// # Ok::<(), regex::Error>(())
    /// ```
    pub fn token<F>(mut self, re: &'r str, f: F) -> Self
    where
        F: Fn(&'t str) -> Option<T> + 'static,
    {
        self.regexes.push(re);
        self.fns.push(Box::new(f));
        self
    }

    /// Construct a [Lexer](struct.Lexer.html) which matches these tokens.
    ///
    /// ## Errors
    ///
    /// If a regex cannot be compiled, a [regex::Error](https://crates.io/regex/struct.Error.html) is returned.
    pub fn build(self) -> Result<Lexer<'t, T>, regex::Error> {
        let regexes = self.regexes.into_iter().map(|r| format!("^{}", r));
        let regex_set = RegexSet::new(regexes)?;
        let mut regexes = Vec::new();
        for pattern in regex_set.patterns() {
            regexes.push(Regex::new(pattern)?);
        }

        Ok(Lexer {
            fns: self.fns,
            regexes,
            regex_set,
        })
    }
}

/// A regex-based lexer.
///
/// ```
/// #[derive(Debug, PartialEq, Eq)]
/// enum Token<'t> {
///     Ident(&'t str),
///     // ...
/// }
///
/// let lexer = regex_lexer::LexerBuilder::new()
///     .token(r"\p{XID_Start}\p{XID_Continue}*", |id| Some(Token::Ident(id)))
///     .token(r"\s+", |_| None) // skip whitespace
///     // ...
///     .build()?;
///
/// let tokens = lexer.tokens("these are some identifiers");
///
/// # assert_eq!(
/// #    tokens.collect::<Vec<_>>(),
/// #    vec![Token::Ident("these"), Token::Ident("are"), Token::Ident("some"), Token::Ident("identifiers")],
/// # );
/// # Ok::<(), regex::Error>(())
/// ```
pub struct Lexer<'t, T: 't> {
    fns: Vec<Box<dyn Fn(&'t str) -> Option<T>>>,
    regexes: Vec<Regex>,
    regex_set: RegexSet,
}

impl<'t, T: 't> Lexer<'t, T> {
    /// Create a [LexerBuilder](struct.LexerBuilder.html). This is the same as [LexerBuilder::new](struct.LexerBuilder.html#method.new).
    pub fn builder<'r>() -> LexerBuilder<'r, 't, T> {
        LexerBuilder::new()
    }

    /// Return an iterator over all matched tokens.
    pub fn tokens<'l>(&'l self, source: &'t str) -> Tokens<'l, 't, T> {
        Tokens {
            lexer: self,
            source,
            position: 0,
        }
    }
}

impl<'t, T: 't> std::fmt::Debug for Lexer<'t, T> {
    /// Shows the original regular expressions
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Lexer")
            .field("regexes", &self.regexes)
            .finish() // todo: finish_non_exhaustive
    }
}

/// The type returned by [Lexer::tokens](struct.Lexer.html#method.tokens).
#[derive(Debug)]
pub struct Tokens<'l, 't, T: 't> {
    lexer: &'l Lexer<'t, T>,
    source: &'t str,
    position: usize,
}

impl<'l, 't, T: 't> Iterator for Tokens<'l, 't, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
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
                .max_by_key(|(len, _)| *len)
                .unwrap();

            let tok_str = &self.source[self.position..self.position + len];
            self.position += len;
            let func = &mut self.lexer.fns[i];
            match func(tok_str) {
                Some(tok) => return Some(tok),
                None => {}
            }
        }
    }
}
