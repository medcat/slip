use super::expression::Expression;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node};
use serde_derive::*;

mod catch;
mod for_;
mod group;
mod if_;
mod let_;
mod return_;
mod try_;
mod unless;
mod while_;

pub use self::catch::Catch;
pub use self::for_::For;
pub use self::group::StatementGroup;
pub use self::if_::{If, IfCondition};
pub use self::let_::Let;
pub use self::return_::Return;
pub use self::try_::Try;
pub use self::unless::Unless;
pub use self::while_::While;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Unless(Box<Unless>),
    Return(Box<Return>),
    While(Box<While>),
    For(Box<For>),
    // Use(Use),
    Let(Box<Let>),
    Try(Box<Try>),
    If(If),
    Expression(Expression),
}

impl Node for Statement {
    fn parse(stream: &mut TokenStream) -> Result<Statement, Error> {
        match stream.peek_kind() {
            Some(TokenKind::Unless) => Ok(Statement::Unless(Box::new(Unless::parse(stream)?))),
            Some(TokenKind::Return) => Ok(Statement::Return(Box::new(Return::parse(stream)?))),
            Some(TokenKind::While) => Ok(Statement::While(Box::new(While::parse(stream)?))),
            Some(TokenKind::For) => Ok(Statement::For(Box::new(For::parse(stream)?))),
            Some(TokenKind::Let) => Ok(Statement::Let(Box::new(Let::parse(stream)?))),
            Some(TokenKind::Try) => Ok(Statement::Try(Box::new(Try::parse(stream)?))),
            Some(TokenKind::If) => Ok(Statement::If(If::parse(stream)?)),
            _ => {
                let expr = Expression::parse(stream)?;
                stream.expect_one(TokenKind::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
        }
    }
}

impl BasicNode for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::Unless(stmt) => stmt.span(),
            Statement::Return(stmt) => stmt.span(),
            Statement::While(stmt) => stmt.span(),
            Statement::For(stmt) => stmt.span(),
            // Statement::Use(stmt) => stmt.span(),
            Statement::Let(stmt) => stmt.span(),
            Statement::Try(stmt) => stmt.span(),
            Statement::If(stmt) => stmt.span(),
            Statement::Expression(stmt) => stmt.span(),
        }
    }
}
