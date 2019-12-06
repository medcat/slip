use super::super::FunctionName;
use super::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Access(pub(super) Box<Expression>, pub(super) FunctionName, Span);

impl Access {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<Access, Error> {
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
