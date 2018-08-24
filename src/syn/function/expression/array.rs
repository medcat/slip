use super::Expression;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Array(Roll<Expression>);

impl Node for Array {
    fn parse(stream: &mut TokenStream) -> Result<Array> {
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
