#[cfg(test)]
mod test;
mod token;

pub use self::token::{Token, TokenKind};
use super::diag::*;
use crate::error::Error;

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
pub struct TokenStream<'d> {
    file: &'d File,
    offset: usize,
    position: Position,
    diag: &'d mut DiagnosticSet,
    next: Option<Result<Token, Error>>,
}

impl<'d> TokenStream<'d> {
    /// Creates a new lexer from the given source.
    pub fn new(file: &'d File, diag: &'d mut DiagnosticSet) -> TokenStream<'d> {
        TokenStream {
            file,
            offset: 0,
            position: Position::default(),
            diag,
            next: None,
        }
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
        Span::new(self.position, self.position, Some(self.file.id()))
    }

    /// Peeks into the next token.  This is useful for doing a
    /// lookahead, without advancing the iterator.  This follows
    /// the same semantics as [`std::iter::Peekable::peek`], except
    /// you don't have to call [`std::iter::Iterator::peekable`].
    pub fn peek(&mut self) -> Option<&Result<Token, Error>> {
        if self.next.is_none() {
            let result = self.next();
            self.next = result;
        }

        self.next.as_ref()
    }

    /// Expects any of the given tokens; if the next token one isn't
    /// provided, or the iterator returns `None`, this errors, detailing
    /// what token was given, what tokens were expected, and where to
    /// find them.  Note that this passes through errors from the
    /// iterator.
    pub fn expect_any(&mut self, kinds: &[TokenKind]) -> Result<Token, Error> {
        match self.next() {
            Some(Ok(token)) => {
                if kinds.contains(&token.kind) {
                    Ok(token)
                } else {
                    error(self.diag, token.kind, kinds, token.span)
                        .map(|t| t)
                }
            }
            Some(Err(e)) => Err(e),
            None => error(self.diag, TokenKind::Eof, kinds, self.span()).map(|t| t),
        }
    }

    /// This is an optimised version of [`TokenStream::expect_any`].  This
    /// takes one token kind, and if the next token isn't that token, it
    /// errors; or, if there are no more tokens, it errors.  Note that this
    /// passes errors through from the iterator.
    pub fn expect_one(&mut self, kind: TokenKind) -> Result<Token, Error> {
        match self.next() {
            Some(Ok(token)) => {
                if token.kind == kind {
                    Ok(token)
                } else {
                    error(self.diag, token.kind, &[kind], token.span)
                        .map(|t| t)
                }
            }
            Some(Err(e)) => Err(e),
            None => error(self.diag, TokenKind::Eof, &[kind], self.span()).map(|t| t),
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

    /// This is an optimised version of [`TokenStream::peek_any`].  This takes
    /// one token kind, and checks if the next token is that kind; if it is,
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
    /// ```
    /// lexer.error_from(&[TokenKind::Module])
    /// ```
    pub fn error_from(&mut self, expected: &[TokenKind]) -> Result<!, Error> {
        let next = self.next();
        match next {
            Some(Ok(token)) => error(self.diag, token.kind, expected, token.span),
            Some(Err(e)) => Err(e),
            None => error(self.diag, TokenKind::Eof, expected, self.span()),
        }
    }

    pub fn rolling<F, T, E: From<Error>>(
        &mut self,
        end: Option<TokenKind>,
        seperator: TokenKind,
        at_least: bool,
        trailing: bool,
        mut func: F,
    ) -> Result<Vec<T>, E>
    where
        F: FnMut(&mut Self) -> Result<T, E>,
    {
        let terminating = |lex: &mut TokenStream| match end {
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
    /// iterator is guarenteed to return [`None`].  Note that this
    /// requires a mutable reference because it _does_ advance the
    /// iterator - if you need to check without advancing the
    /// iterator, first clone, then call this.  Since we skip specific
    /// tokens, we can't know if we're at the EOF without checking what
    /// the next token would be.
    pub fn eof(&mut self) -> bool {
        self.peek().is_none()
    }
}

impl<'d> Iterator for TokenStream<'d> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_some() {
            return self.next.take();
        }

        if self.offset >= self.file.content().len() {
            return None;
        }

        let point = &self.file.content()[self.offset..];

        match find_next(point) {
            Some((kind, mat)) => {
                let value = mat.as_str();
                // let offset = self.1 + value.len();
                let line = self.position.line() + value.match_indices('\n').count();
                let column = value
                    .rfind('\n')
                    .map(|v| value.len() - v)
                    .unwrap_or(self.position.column() + value.len());
                let position = Position::new(line, column);
                let span = Span::new(self.position, position, Some(self.file.id()));
                self.offset += value.len();
                self.position = position;
                let token = Token::new(kind, span, Some(value));

                if kind.ignore() {
                    self.next()
                } else {
                    Some(Ok(token))
                }
            }
            None => {
                self.offset = self.file.content().len();
                let c = point.chars().next().unwrap_or('\u{fffd}');
                Some(Err(Error::UnexpectedSymbolError {
                    symbol: c,
                    line: self.position.line(),
                    column: self.position.column(),
                }))
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
        })
        .max_by(|(_, ai, am), (_, bi, bm)| {
            am.as_str().len().cmp(&bm.as_str().len()).then(bi.cmp(ai))
        })
        .map(|(k, _, m)| (k, m))
}

fn error(diag: &mut DiagnosticSet, current: TokenKind, expected: &[TokenKind], span: Span) -> Result<!, Error> {
    diag.emit(Diagnostic::UnexpectedToken, span, format!("found token {}, expected one of {:?}", current, expected))?;
    Err(Error::UnexpectedTokenError {
        current,
        expected: expected.to_owned(),
        area: span,
    })
}
