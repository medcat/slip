use super::Expression;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index(Box<Expression>, Roll<Expression>, Span);

impl Index {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<Index> {
        let arguments = Roll::with_terminate_trail(
            stream,
            TokenKind::LeftBrace,
            TokenKind::Comma,
            TokenKind::RightBrace,
        )?;
        let span = left.span() | arguments.span();

        Ok(Index(Box::new(left), arguments, span))
    }
}

impl BasicNode for Index {
    fn span(&self) -> Span {
        self.2
    }
}
