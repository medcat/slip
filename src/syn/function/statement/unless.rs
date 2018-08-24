use super::super::expression::Expression;
use super::group::StatementGroup;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unless(Expression, StatementGroup, Span);

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
