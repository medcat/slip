use super::{Expression, Precedence};
use diag::Span;
use error::*;
use stream::{Token, TokenStream};
use syn::BasicNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfixOperation(Box<Expression>, Token, Box<Expression>, Span);

impl InfixOperation {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<InfixOperation> {
        let op = stream.next().unwrap().unwrap();
        let prec: Precedence = op.kind.into();
        let right = Expression::parse_prec(stream, prec)?;
        let span = left.span() | op.span() | right.span();
        Ok(InfixOperation(Box::new(left), op, Box::new(right), span))
    }
}

impl BasicNode for InfixOperation {
    fn span(&self) -> Span {
        self.3
    }
}
