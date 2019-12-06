use super::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Array(Roll<Expression>);

impl Node for Array {
    fn parse(stream: &mut TokenStream) -> Result<Array, Error> {
        let contents = Roll::with_terminate_trail(
            stream,
            TokenKind::LeftBracket,
            TokenKind::Comma,
            TokenKind::RightBracket,
        )?;
        Ok(Array(contents))
    }
}

impl BasicNode for Array {
    fn span(&self) -> Span {
        self.0.span()
    }
}
