use super::super::FunctionName;
use super::{Atom, Expression};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use crate::syn::{BasicNode, Roll};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Call {
    Unified(Box<Unified>),
    Standard(Box<Standard>),
    Expression(Box<Expr>),
}

impl Call {
    pub fn parse(stream: &mut TokenStream, left: Expression) -> Result<Call, Error> {
        let arguments = Roll::with_terminate_trail(
            stream,
            TokenKind::LeftParen,
            TokenKind::Comma,
            TokenKind::RightParen,
        )?;
        let span = left.span() | arguments.span();

        match left {
            Expression::Access(access) => Ok(Call::Unified(Box::new(Unified::new(
                *access.0, access.1, arguments, span,
            )))),
            Expression::Atom(Atom::Ident(tok)) => Ok(Call::Standard(Box::new(Standard::new(
                tok, arguments, span,
            )))),
            v => Ok(Call::Expression(Box::new(Expr::new(v, arguments, span)))),
        }
    }
}

impl BasicNode for Call {
    fn span(&self) -> Span {
        match self {
            Call::Unified(un) => un.span,
            Call::Standard(std) => std.span,
            Call::Expression(expr) => expr.span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unified {
    pub base: Expression,
    pub func: FunctionName,
    pub params: Roll<Expression>,
    pub span: Span,
}

impl Unified {
    fn new(base: Expression, func: FunctionName, params: Roll<Expression>, span: Span) -> Unified {
        Unified {
            base,
            func,
            params,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standard {
    pub token: Token,
    pub params: Roll<Expression>,
    pub span: Span,
}

impl Standard {
    fn new(token: Token, params: Roll<Expression>, span: Span) -> Standard {
        Standard {
            token,
            params,
            span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expr {
    pub base: Expression,
    pub params: Roll<Expression>,
    pub span: Span,
}

impl Expr {
    fn new(base: Expression, params: Roll<Expression>, span: Span) -> Expr {
        Expr { base, params, span }
    }
}
