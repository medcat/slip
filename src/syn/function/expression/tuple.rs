use super::Expression;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tuple(Roll<Expression>);

impl Node for Tuple {
    fn parse(stream: &mut TokenStream) -> Result<Tuple> {
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
