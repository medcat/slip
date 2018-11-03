use super::super::FunctionName;
use super::{Atom, Expression};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use crate::syn::{BasicNode, Roll};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Call {
    Unified(Box<Expression>, FunctionName, Roll<Expression>, Span),
    Standard(Token, Roll<Expression>, Span),
    Expression(Box<Expression>, Roll<Expression>, Span),
}

impl Call {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<Call> {
        let arguments = Roll::with_terminate_trail(
            stream,
            TokenKind::LeftParen,
            TokenKind::Comma,
            TokenKind::RightParen,
        )?;
        let span = left.span() | arguments.span();

        match left {
            Expression::Access(access) => Ok(Call::Unified(access.0, access.1, arguments, span)),
            Expression::Atom(Atom::Ident(tok)) => Ok(Call::Standard(tok, arguments, span)),
            v => Ok(Call::Expression(Box::new(v), arguments, span)),
        }
    }
}

impl BasicNode for Call {
    fn span(&self) -> Span {
        match self {
            Call::Unified(_, _, _, span) => *span,
            Call::Standard(_, _, span) => *span,
            Call::Expression(_, _, span) => *span,
        }
    }
}
