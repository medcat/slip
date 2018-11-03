use super::super::expression::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use crate::syn::{BasicNode, Node, Type};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Let(Token, Option<Type>, Option<Expression>, Span);

impl Let {
    pub fn token(&self) -> &Token {
        &self.0
    }
    pub fn token_mut(&mut self) -> &mut Token {
        &mut self.0
    }

    pub fn kind(&self) -> &Option<Type> {
        &self.1
    }

    pub fn kind_mut(&mut self) -> &mut Option<Type> {
        &mut self.1
    }

    pub fn value(&self) -> &Option<Expression> {
        &self.2
    }

    pub fn value_mut(&mut self) -> &mut Option<Expression> {
        &mut self.2
    }
}

impl Node for Let {
    fn parse(stream: &mut TokenStream) -> Result<Let> {
        let mut span = stream.expect_one(TokenKind::Let)?.span();
        let name = stream.expect_one(TokenKind::Identifier)?;
        span |= name.span();
        let kind = if stream.peek_one(TokenKind::Colon) {
            span |= stream.expect_one(TokenKind::Colon)?.span();
            let kind = Type::parse(stream)?;
            span |= kind.span();
            Some(kind)
        } else {
            None
        };
        let value = if stream.peek_one(TokenKind::Equals) {
            span |= stream.expect_one(TokenKind::Equals)?.span();
            let expr = Expression::parse(stream)?;
            span |= expr.span();
            Some(expr)
        } else {
            None
        };
        Ok(Let(name, kind, value, span))
    }
}

impl BasicNode for Let {
    fn span(&self) -> Span {
        self.3
    }
}
