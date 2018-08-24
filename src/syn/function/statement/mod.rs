use super::expression::Expression;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

mod catch;
mod for_;
mod group;
mod if_;
mod let_;
mod return_;
mod try;
mod unless;
mod while_;

pub use self::catch::Catch;
pub use self::for_::For;
pub use self::group::StatementGroup;
pub use self::if_::{If, IfCondition};
pub use self::let_::Let;
pub use self::return_::Return;
pub use self::try::Try;
pub use self::unless::Unless;
pub use self::while_::While;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Unless(Unless),
    Return(Return),
    While(While),
    For(For),
    // Use(Use),
    Let(Let),
    Try(Try),
    If(If),
    Expression(Expression),
}

impl Node for Statement {
    fn parse(stream: &mut TokenStream) -> Result<Statement> {
        match stream.peek_kind() {
            Some(TokenKind::While) => Ok(Statement::While(While::parse(stream)?)),
            Some(TokenKind::Try) => Ok(Statement::Try(Try::parse(stream)?)),
            Some(TokenKind::For) => Ok(Statement::For(For::parse(stream)?)),
            // Some(TokenKind::Use) => Ok(Statement::Use(Use::parse(stream)?)),
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
