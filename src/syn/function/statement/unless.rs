use super::super::expression::Expression;
use super::group::StatementGroup;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unless(Expression, StatementGroup, Span);

impl Unless {
    pub fn condition(&self) -> &Expression {
        &self.0
    }
    pub fn condition_mut(&mut self) -> &mut Expression {
        &mut self.0
    }
    pub fn body(&self) -> &StatementGroup {
        &self.1
    }
    pub fn body_mut(&mut self) -> &mut StatementGroup {
        &mut self.1
    }
}

impl Node for Unless {
    fn parse(stream: &mut TokenStream) -> Result<Unless> {
        let mut span = stream.expect_one(TokenKind::Unless)?.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let body = StatementGroup::parse(stream)?;
        span |= body.span();

        Ok(Unless(condition, body, span))
    }
}

impl BasicNode for Unless {
    fn span(&self) -> Span {
        self.2
    }
}
