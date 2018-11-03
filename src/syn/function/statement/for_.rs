use super::super::expression::Expression;
use super::group::StatementGroup;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use crate::syn::{BasicNode, Node};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct For(Token, Expression, StatementGroup, Span);

impl For {
    pub fn local(&self) -> &Token {
        &self.0
    }
    pub fn local_mut(&mut self) -> &mut Token {
        &mut self.0
    }
    pub fn iterator(&self) -> &Expression {
        &self.1
    }
    pub fn iterator_mut(&mut self) -> &mut Expression {
        &mut self.1
    }
    pub fn body(&self) -> &StatementGroup {
        &self.2
    }
    pub fn body_mut(&mut self) -> &mut StatementGroup {
        &mut self.2
    }
}

impl Node for For {
    fn parse(stream: &mut TokenStream) -> Result<For> {
        let mut span = stream.expect_one(TokenKind::For)?.span();
        let token = stream.expect_one(TokenKind::Identifier)?;
        span |= token.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let body = StatementGroup::parse(stream)?;
        span |= body.span();

        Ok(For(token, condition, body, span))
    }
}

impl BasicNode for For {
    fn span(&self) -> Span {
        self.3
    }
}
