use super::{Expression, Precedence};
use diag::Span;
use error::*;
use stream::{Token, TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixOperation(Box<Expression>, Token, Span);

impl Node for PrefixOperation {
    fn parse(stream: &mut TokenStream) -> Result<PrefixOperation> {
        let op = stream.expect_any(&[
            TokenKind::DoublePlus,
            TokenKind::DoubleMinus,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::LogicalNot,
            TokenKind::BitwiseNot,
        ])?;
        let right = Expression::parse_prec(stream, Precedence::PrefixPlusLogical)?;
        let span = op.span() | right.span();
        Ok(PrefixOperation(Box::new(right), op, span))
    }
}

impl BasicNode for PrefixOperation {
    fn span(&self) -> Span {
        self.2
    }
}
