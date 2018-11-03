use super::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use crate::syn::BasicNode;
use serde_derive::*;

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
