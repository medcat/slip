#[cfg(test)]
mod test;
mod token;

pub use self::token::{Token, TokenKind};
use super::diag::*;
use error::*;

#[derive(Debug)]
/// Performs the lexical analysis on a given string.  This implements
/// the [`Iterator`] trait directly, yielding a [`Result`] of
/// [`Token`]s.  Typically, this shouldn't error save on invalid input.
/// Once it errors, it no longer yields any more tokens.  The tokens
/// themselves will have references to the original text; however, each
/// token can have its reference revoked.  See [`Token`] for more
/// details.
///
/// # Example
///
/// ```rust
/// # fn main() {
/// let lexer = TokenStream::new("return Int::default();");
/// let tokens = lexer.map(|tok| tok.unwrap().kind()).collect::<Vec<_>>();
/// assert_eq!(tokens, vec![TokenKind::Return,
///     TokenKind::Whitespace, TokenKind::ModuleName,
///     TokenKind::DoubleColon, TokenKind::Identifier,
///     TokenKind::LeftParen, TokenKind::RightParen,
///     TokenKind::Semicolon]);
/// # }
/// ```
pub struct TokenStream<'a>(&'a str, usize, Position, Option<Result<Token>>);

impl<'a> TokenStream<'a> {
    /// Creates a new lexer from the given source.  The TokenStream's
    /// lifetime, and the resulting tokens, are tied to this source.
    pub fn new(source: &'a str) -> TokenStream<'a> {
        TokenStream(source, 0, Position::default(), None)
    }

    /// Retrieves the span of where the lexer is.  By the nature of
    /// a span, this means that this span has no width - it is where
    /// the lexer is in terms of parsing.  This is useful for
    /// generating a terminal token, to create e.g. a message.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() {
    /// let lexer = TokenStream::new("hello");
    /// assert_eq!(lexer.span(), Span::new(Position::default(),
    ///     Position::default()));
    /// # }
    /// ```
    pub fn span(&self) -> Span {
        Span::new(self.2, self.2)
    }

    /// Peeks into the next token.  This is useful for doing a
    /// lookahead, without advancing the iterator.  This follows
    /// the same semantics as [`std::iter::Peekable::peek`], except
    /// you don't have to call [`std::iter::Iterator::peekable`].
    pub fn peek(&mut self) -> Option<&Result<Token>> {
        if self.3.is_none() {
            let result = self.next();
            self.3 = result;
        }

        self.3.as_ref()
    }

    /// Expects any of the given tokens; if the next token one isn't
    /// provided, or the iterator returns `None`, this errors, detailing
    /// what token was given, what tokens were expected, and where to
    /// find them.  Note that this passes through errors from the
    /// iterator.
    pub fn expect_any(&mut self, kinds: &[TokenKind]) -> Result<Token> {
        match self.next() {
            Some(Ok(token)) => {
                if kinds.contains(&token.kind) {
                    Ok(token)
                } else {
                    error(token.kind, kinds, token.span)
                }
            }
            Some(Err(e)) => Err(e),
            None => error(TokenKind::Eof, kinds, self.span()),
        }
    }

    /// This is an optimised version of [`expect_any`].  This takes one
    /// token kind, and if the next token isn't that token, it errors;
    /// or, if there are no more tokens, it errors.  Note that this
    /// passes errors through from the iterator.
    pub fn expect_one(&mut self, kind: TokenKind) -> Result<Token> {
        match self.next() {
            Some(Ok(token)) => {
                if token.kind == kind {
                    Ok(token)
                } else {
                    error(token.kind, &[kind], token.span)
                }
            }
            Some(Err(e)) => Err(e),
            None => error(TokenKind::Eof, &[kind], self.span()),
        }
    }

    /// This returns the type of the next token, if there is a next
    /// token, and it is not an error.  This can be used to match on
    /// behavior.
    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.peek().and_then(|v| v.as_ref().ok()).map(|v| v.kind)
    }

    /// Peeks at the next token; if there is a token, and it is one of
    /// the given token types, this returns true.  If there is no more
    /// tokens, or the next value is an error, or the next token isn't
    /// one of the given kinds, then this returns false.
    pub fn peek_any(&mut self, kinds: &[TokenKind]) -> bool {
        match self.peek() {
            Some(Ok(token)) if kinds.contains(&token.kind) => true,
            _ => false,
        }
    }

    /// This is an optimised version of [`peek_any`].  This takes one
    /// token kind, and checks if the next token is that kind; if it is,
    /// it return true.  If there is no more tokens, or the next value
    /// is an error, or te next token isn't of the given kind, it
    /// returns false.
    pub fn peek_one(&mut self, kind: TokenKind) -> bool {
        match self.peek() {
            Some(Ok(token)) if token.kind == kind => true,
            _ => false,
        }
    }

    /// Generates an error.  This *never* returns the Ok variant of the
    /// result.  This consumes the next token; if the next token is available,
    /// and is not an error, it generates an error, with that token's kind
    /// and position as the given kind and position for the error, and the
    /// expected tokens as the expected for the error.  If the next value is
    /// an error, it passes that through.  If there are no more tokens, it
    /// creates a new error, with [`TokenKind::Eof`] as the given kind, the
    /// current position as the location (see [`TokenStream::span`]), and the
    /// expected tokens as the expected for the error.
    ///
    /// # Notes
    /// The [`Result::Ok`] variant is _never_ created using this funtion.
    /// Despite that, the Ok variant is declared to be the unit type.
    /// Don't let that fool you - the never type (`!`) is currently
    /// unstable, and if it was stable, I would be using that here
    /// instead.  In the meantime, you can just do something like this:
    ///
    /// ```
    /// lexer.error_from(&[TokenKind::Module]).map(|_| unreachable!())
    /// ```
    pub fn error_from(&mut self, expected: &[TokenKind]) -> Result<()> {
        let next = self.next();
        match next {
            Some(Ok(token)) => error(token.kind, expected, token.span),
            Some(Err(e)) => Err(e),
            None => error(TokenKind::Eof, expected, self.span()),
        }
    }

    pub fn rolling<F, T>(
        &mut self,
        end: Option<TokenKind>,
        seperator: TokenKind,
        at_least: bool,
        trailing: bool,
        mut func: F,
    ) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let terminating = |lex: &mut TokenStream<'a>| match end {
            Some(v) => lex.peek_one(v),
            _ => !lex.peek_one(seperator),
        };
        let first = if at_least || !terminating(self) {
            func(self)?
        } else {
            if let Some(v) = end {
                self.expect_one(v)?;
            }
            return Ok(vec![]);
        };

        let mut body = vec![first];

        while !terminating(self) {
            self.expect_one(seperator)?;
            if trailing && end.map(|v| self.peek_one(v)).unwrap_or(false) {
                break;
            } else {
                body.push(func(self)?);
            }
        }

        if let Some(v) = end {
            self.expect_one(v)?;
        }

        Ok(body)
    }

    /// Whether or not the lexer is at EOF.  If this is true, the
    /// iterator is guarenteed to return `None`.  Note that this
    /// requires a mutable reference because it _does_ advance the
    /// iterator - if you need to check without advancing the
    /// iterator, first clone, then call this.  Since we skip specific
    /// tokens, we can't know if we're at the EOF without checking what
    /// the next token would be.
    pub fn eof(&mut self) -> bool {
        match self.peek() {
            Some(_) => false,
            None => true,
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.3.is_some() {
            return self.3.take();
        }

        if self.1 >= self.0.len() {
            return None;
        }

        let point = &self.0[self.1..];

        match find_next(point) {
            Some((kind, mat)) => {
                let value = mat.as_str();
                // let offset = self.1 + value.len();
                let line = self.2.line() + value.match_indices('\n').count();
                let column = value
                    .rfind('\n')
                    .map(|v| value.len() - v)
                    .unwrap_or(self.2.column() + value.len());
                let position = Position::new(line, column);
                let span = Span::new(self.2, position);
                self.1 += value.len();
                self.2 = position;
                let token = Token::new(kind, span, Some(value));

                if kind.ignore() {
                    self.next()
                } else {
                    Some(Ok(token))
                }
            }
            None => {
                self.1 = self.0.len();
                let c = point.chars().next().unwrap_or('\u{fffd}');
                Some(Err(ErrorKind::UnexpectedSymbolError(c, self.2).into()))
            }
        }
    }
}

fn find_next(source: &str) -> Option<(TokenKind, ::regex::Match)> {
    TokenKind::set()
        .matches(source)
        .into_iter()
        .flat_map(|idx| {
            let kind = TokenKind::tokens()[idx];
            let pattern = &TokenKind::value()[idx];
            pattern.find(source).map(|m| (kind, idx, m))
        }).max_by(|(_, ai, am), (_, bi, bm)| {
            am.as_str().len().cmp(&bm.as_str().len()).then(bi.cmp(ai))
        }).map(|(k, _, m)| (k, m))
}

fn error<R>(current: TokenKind, expected: &[TokenKind], span: Span) -> Result<R> {
    Err(ErrorKind::UnexpectedTokenError(current, expected.to_owned(), span).into())
}
