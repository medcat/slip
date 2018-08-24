use super::group::StatementGroup;
use diag::Span;
use error::*;
use stream::{Token, TokenKind, TokenStream};
use syn::{BasicNode, Node, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catch {
    local: Token,
    kind: Type,
    body: StatementGroup,
    area: Span,
}

impl Node for Catch {
    fn parse(stream: &mut TokenStream) -> Result<Catch> {
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
