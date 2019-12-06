use crate::diag::Span;
use crate::stream::TokenKind;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "unexpected token {}, expected one of {:?} at {}",
        current, expected, area
    )]
    UnexpectedTokenError {
        current: TokenKind,
        expected: Vec<TokenKind>,
        area: Span,
    },

    #[fail(
        display = "unexpected symbol {:x?} occurred at line {}, column {}",
        symbol, line, column
    )]
    UnexpectedSymbolError {
        symbol: char,
        line: usize,
        column: usize,
    },

    #[fail(display = "encountered an io exception")]
    IoError(::std::io::Error),
}

impl From<::std::io::Error> for Error {
    fn from(er: ::std::io::Error) -> Error {
        Error::IoError(er)
    }
}
