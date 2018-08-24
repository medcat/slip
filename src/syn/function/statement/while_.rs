use super::super::expression::Expression;
use super::group::StatementGroup;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct While(Expression, StatementGroup, Span);

impl Node for While {
    fn parse(stream: &mut TokenStream) -> Result<While> {
        let mut span = stream.expect_one(TokenKind::While)?.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let body = StatementGroup::parse(stream)?;
        span |= body.span();

        Ok(While(condition, body, span))
    }
}

impl BasicNode for While {
    fn span(&self) -> Span {
        self.2
    }
}
