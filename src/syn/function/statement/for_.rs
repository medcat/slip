use super::super::expression::Expression;
use super::group::StatementGroup;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct For(Expression, StatementGroup, Span);

impl Node for For {
    fn parse(stream: &mut TokenStream) -> Result<For> {
        let mut span = stream.expect_one(TokenKind::For)?.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let body = StatementGroup::parse(stream)?;
        span |= body.span();

        Ok(For(condition, body, span))
    }
}

impl BasicNode for For {
    fn span(&self) -> Span {
        self.2
    }
}
