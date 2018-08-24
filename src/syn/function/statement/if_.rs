use super::super::expression::Expression;
use super::group::StatementGroup;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If(Vec<IfCondition>, Span);

impl Node for If {
    fn parse(stream: &mut TokenStream) -> Result<If> {
        let mut span = stream.expect_one(TokenKind::If)?.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let base = StatementGroup::parse(stream)?;
        span |= base.span();
        let mut conditionals = vec![IfCondition(Some(condition), base, span.clone())];
        while stream.peek_any(&[TokenKind::Elsif, TokenKind::Else]) {
            let cond = IfCondition::parse(stream)?;
            span |= cond.span();
            conditionals.push(cond);
        }

        Ok(If(conditionals, span))
    }
}

impl BasicNode for If {
    fn span(&self) -> Span {
        self.1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfCondition(Option<Expression>, StatementGroup, Span);

impl IfCondition {
    fn parse_elsif(stream: &mut TokenStream) -> Result<IfCondition> {
        let mut span = stream.expect_one(TokenKind::Elsif)?.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let base = StatementGroup::parse(stream)?;
        span |= base.span();
        Ok(IfCondition(Some(condition), base, span))
    }

    fn parse_else(stream: &mut TokenStream) -> Result<IfCondition> {
        let mut span = stream.expect_one(TokenKind::Else)?.span();
        let base = StatementGroup::parse(stream)?;
        span |= base.span();
        Ok(IfCondition(None, base, span))
    }
}

impl Node for IfCondition {
    fn parse(stream: &mut TokenStream) -> Result<IfCondition> {
        match stream.peek_kind() {
            Some(TokenKind::Elsif) => IfCondition::parse_elsif(stream),
            Some(TokenKind::Else) => IfCondition::parse_else(stream),
            _ => stream
                .error_from(&[TokenKind::Elsif, TokenKind::Else])
                .map(|_| unreachable!()),
        }
    }
}

impl BasicNode for IfCondition {
    fn span(&self) -> Span {
        self.2
    }
}
