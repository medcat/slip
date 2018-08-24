use super::Expression;
use diag::Span;
use error::*;
use stream::{Token, TokenKind, TokenStream};
use syn::BasicNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuffixOperation(Box<Expression>, Token, Span);

impl SuffixOperation {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<SuffixOperation> {
        let op = stream.expect_any(&[TokenKind::DoublePlus, TokenKind::DoubleMinus])?;
        let span = left.span() | op.span();
        Ok(SuffixOperation(Box::new(left), op, span))
    }
}

impl BasicNode for SuffixOperation {
    fn span(&self) -> Span {
        self.2
    }
}
