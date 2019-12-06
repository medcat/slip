use super::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tuple(Roll<Expression>);

impl Node for Tuple {
    fn parse(stream: &mut TokenStream) -> Result<Tuple, Error> {
        let contents = Roll::with_terminate_trail(
            stream,
            TokenKind::LeftParen,
            TokenKind::Comma,
            TokenKind::RightParen,
        )?;
        Ok(Tuple(contents))
    }
}

impl BasicNode for Tuple {
    fn span(&self) -> Span {
        self.0.span()
    }
}
