use super::diag::{Position, Span};
use super::stream::TokenKind;
use error_chain::*;

error_chain! {
    errors {
        UnexpectedSymbolError(symbol: char, position: Position) {
            description("unexpected symbol found")
            display("unexpected symbol {:?} found at line {}, column {}", symbol, position.line(), position.column())
        }

        UnexpectedTokenError(found: TokenKind, expected: Vec<TokenKind>, span: Span) {
            description("unexpected token found")
            display("unexpected token {}, expected one of {{{}}}, at {}", found,
                expected.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "), span)
        }

        MissingMainError {

        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
