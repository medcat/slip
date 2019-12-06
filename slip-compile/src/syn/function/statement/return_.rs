use super::super::expression::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return(Option<Expression>, Span);

impl Return {
    pub fn value(&self) -> &Option<Expression> {
        &self.0
    }
    pub fn value_mut(&mut self) -> &mut Option<Expression> {
        &mut self.0
    }
}

impl Node for Return {
    fn parse(stream: &mut TokenStream) -> Result<Return, Error> {
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
