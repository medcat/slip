use super::super::expression::Expression;
use super::group::StatementGroup;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If(Vec<IfCondition>, Span);

impl If {
    pub fn conditions(&self) -> &[IfCondition] {
        &self.0[..]
    }

    pub fn conditions_mut(&mut self) -> &mut [IfCondition] {
        &mut self.0[..]
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &IfCondition> + 'a {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut IfCondition> + 'a {
        self.0.iter_mut()
    }
}

impl Node for If {
    fn parse(stream: &mut TokenStream) -> Result<If, Error> {
        let mut span = stream.expect_one(TokenKind::If)?.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let base = StatementGroup::parse(stream)?;
        span |= base.span();
        let mut conditionals = vec![IfCondition(Some(condition), base, span)];
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

impl<'a> IntoIterator for &'a If {
    type Item = &'a IfCondition;
    type IntoIter = ::std::slice::Iter<'a, IfCondition>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut If {
    type Item = &'a mut IfCondition;
    type IntoIter = ::std::slice::IterMut<'a, IfCondition>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a> IntoIterator for If {
    type Item = IfCondition;
    type IntoIter = <Vec<IfCondition> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfCondition(Option<Expression>, StatementGroup, Span);

impl IfCondition {
    pub fn condition(&self) -> Option<&Expression> {
        self.0.as_ref()
    }

    pub fn condition_mut(&mut self) -> Option<&mut Expression> {
        self.0.as_mut()
    }

    pub fn body(&self) -> &StatementGroup {
        &self.1
    }

    pub fn body_mut(&mut self) -> &mut StatementGroup {
        &mut self.1
    }
}

impl IfCondition {
    fn parse_elsif(stream: &mut TokenStream) -> Result<IfCondition, Error> {
        let mut span = stream.expect_one(TokenKind::Elsif)?.span();
        let condition = Expression::parse(stream)?;
        span |= condition.span();
        let base = StatementGroup::parse(stream)?;
        span |= base.span();
        Ok(IfCondition(Some(condition), base, span))
    }

    fn parse_else(stream: &mut TokenStream) -> Result<IfCondition, Error> {
        let mut span = stream.expect_one(TokenKind::Else)?.span();
        let base = StatementGroup::parse(stream)?;
        span |= base.span();
        Ok(IfCondition(None, base, span))
    }
}

impl Node for IfCondition {
    fn parse(stream: &mut TokenStream) -> Result<IfCondition, Error> {
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
