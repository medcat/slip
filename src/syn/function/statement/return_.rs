use super::super::expression::Expression;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return(Option<Expression>, Span);

impl Node for Return {
    fn parse(stream: &mut TokenStream) -> Result<Return> {
        let mut span = stream.expect_one(TokenKind::Return)?.span();
        let value = if !stream.peek_one(TokenKind::Semicolon) {
            let expr = Expression::parse(stream)?;
            span |= expr.span();
            Some(expr)
        } else {
            None
        };
        span |= stream.expect_one(TokenKind::Semicolon)?.span();
        Ok(Return(value, span))
    }
}

impl BasicNode for Return {
    fn span(&self) -> Span {
        self.1
    }
}
