use super::function::expression::Expression;
use super::{BasicNode, Node, Roll, Type};
use crate::diag::Span;
use crate::error::Error;
use crate::stream::{Token, TokenKind, TokenStream};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum(Type, Roll<EnumVariant>, Span);

impl Enum {
    pub fn kind(&self) -> &Type {
        &self.0
    }

    pub fn variants(&self) -> &[EnumVariant] {
        self.1.value()
    }
}

impl Node for Enum {
    fn parse(stream: &mut TokenStream) -> Result<Enum, Error> {
        let mut span = stream.expect_one(TokenKind::Enum)?.span();
        let kind = Type::parse(stream)?;
        span |= kind.span();

        let contents = Roll::with_terminate_trail_once(
            stream,
            TokenKind::LeftBrace,
            TokenKind::Comma,
            TokenKind::RightBrace,
        )?;

        span |= contents.span();

        Ok(Enum(kind, contents, span))
    }
}

impl BasicNode for Enum {
    fn span(&self) -> Span {
        self.2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnumVariant {
    Name(Token),
    Value(Token, Box<Expression>, Span),
    Unit(Token, Roll<Type>, Span),
}

impl Node for EnumVariant {
    fn parse(stream: &mut TokenStream) -> Result<EnumVariant, Error> {
        let name = stream.expect_one(TokenKind::ModuleName)?;
        let mut span = name.span();

        match stream.peek_kind() {
            Some(TokenKind::Equals) => {
                span |= stream.expect_one(TokenKind::Equals)?.span();
                let expr = Expression::parse(stream)?;
                span |= expr.span();
                Ok(EnumVariant::Value(name, Box::new(expr), span))
            }
            Some(TokenKind::LeftParen) => {
                let contents = Roll::with_terminate_trail_once(
                    stream,
                    TokenKind::LeftParen,
                    TokenKind::Comma,
                    TokenKind::RightParen,
                )?;
                span |= contents.span();
                Ok(EnumVariant::Unit(name, contents, span))
            }
            _ => Ok(EnumVariant::Name(name)),
        }
    }
}

impl BasicNode for EnumVariant {
    fn span(&self) -> Span {
        match self {
            EnumVariant::Value(_, _, span) => *span,
            EnumVariant::Unit(_, _, span) => *span,
            EnumVariant::Name(tok) => tok.span(),
        }
    }
}
