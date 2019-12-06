use super::group::StatementGroup;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use crate::syn::{BasicNode, Node, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catch {
    local: Token,
    kind: Type,
    body: StatementGroup,
    area: Span,
}

impl Catch {
    pub fn local(&self) -> &Token {
        &self.local
    }
    pub fn local_mut(&mut self) -> &mut Token {
        &mut self.local
    }
    pub fn kind(&self) -> &Type {
        &self.kind
    }
    pub fn kind_mut(&mut self) -> &mut Type {
        &mut self.kind
    }
    pub fn body(&self) -> &StatementGroup {
        &self.body
    }
    pub fn body_mut(&mut self) -> &mut StatementGroup {
        &mut self.body
    }
}

impl Node for Catch {
    fn parse(stream: &mut TokenStream) -> Result<Catch, Error> {
        let local = stream.expect_any(&[TokenKind::Underscore, TokenKind::Identifier])?;
        let mut span = local.span();
        let kind = Type::parse(stream)?;
        span |= kind.span();
        let body = StatementGroup::parse(stream)?;
        span |= body.span();

        Ok(Catch {
            local,
            kind,
            body,
            area: span,
        })
    }
}

impl BasicNode for Catch {
    fn span(&self) -> Span {
        self.area
    }
}
