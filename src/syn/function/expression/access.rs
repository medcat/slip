use super::super::FunctionName;
use super::Expression;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Access(pub(super) Box<Expression>, pub(super) FunctionName, Span);

impl Access {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<Access> {
        let mut span = left.span();
        span |= stream.expect_one(TokenKind::Period)?.span();
        let name = FunctionName::parse(stream)?;
        span |= name.span();
        Ok(Access(Box::new(left), name, span))
    }
}

impl BasicNode for Access {
    fn span(&self) -> Span {
        self.2
    }
}
