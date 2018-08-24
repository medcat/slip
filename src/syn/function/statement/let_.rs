use super::super::expression::Expression;
use diag::Span;
use error::*;
use stream::{Token, TokenKind, TokenStream};
use syn::{BasicNode, Node, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Let(Token, Option<Type>, Option<Expression>, Span);

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
