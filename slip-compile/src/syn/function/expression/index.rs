use super::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index(Box<Expression>, Roll<Expression>, Span);

impl Index {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<Index, Error> {
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
