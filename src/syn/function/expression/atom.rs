use super::{Array, Map, Tuple};
use diag::Span;
use error::*;
use stream::{Token, TokenKind, TokenStream};
use syn::{BasicNode, Node, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An atom.  This is the result of parsing a guarenteed unambiguous
/// expression, which must yield a value.
pub enum Atom {
    Underscore(Token),
    Ident(Token),
    SingleString(Token),
    DoubleString(Token),
    Integer(Token),
    Float(Token),
    Type(Type),
    Map(Map),
    Array(Array),
    Tuple(Tuple),
}

impl Node for Atom {
    fn parse(stream: &mut TokenStream) -> Result<Atom> {
        match stream.peek_kind() {
            Some(TokenKind::SingleString) => Ok(Atom::SingleString(
                stream.expect_one(TokenKind::SingleString)?,
            )),
            Some(TokenKind::DoubleString) => Ok(Atom::DoubleString(
                stream.expect_one(TokenKind::DoubleString)?,
            )),
            Some(TokenKind::Integer) => Ok(Atom::Integer(stream.expect_one(TokenKind::Integer)?)),
            Some(TokenKind::Float) => Ok(Atom::Float(stream.expect_one(TokenKind::Float)?)),
            Some(TokenKind::Identifier) => {
                Ok(Atom::Ident(stream.expect_one(TokenKind::Identifier)?))
            }
            Some(TokenKind::Underscore) => {
                Ok(Atom::Underscore(stream.expect_one(TokenKind::Underscore)?))
            }
            Some(TokenKind::ModuleName) => Ok(Atom::Type(Type::parse(stream)?)),
            Some(TokenKind::LeftBrace) => Ok(Atom::Map(Map::parse(stream)?)),
            Some(TokenKind::LeftBracket) => Ok(Atom::Array(Array::parse(stream)?)),
            Some(TokenKind::LeftParen) => Ok(Atom::Tuple(Tuple::parse(stream)?)),
            _ => stream
                .error_from(&[
                    TokenKind::SingleString,
                    TokenKind::DoubleString,
                    TokenKind::Integer,
                    TokenKind::Float,
                    TokenKind::Identifier,
                    TokenKind::Underscore,
                    TokenKind::ModuleName,
                    TokenKind::DoublePlus,
                    TokenKind::DoubleMinus,
                    TokenKind::Plus,
                    TokenKind::Minus,
                    TokenKind::LogicalNot,
                    TokenKind::BitwiseNot,
                    TokenKind::LeftBrace,
                    TokenKind::LeftBracket,
                    TokenKind::LeftParen,
                ]).map(|_| unreachable!()),
        }
    }
}

impl BasicNode for Atom {
    fn span(&self) -> Span {
        match self {
            Atom::Underscore(token) => token.span(),
            Atom::Ident(token) => token.span(),
            Atom::SingleString(token) => token.span(),
            Atom::DoubleString(token) => token.span(),
            Atom::Integer(token) => token.span(),
            Atom::Float(token) => token.span(),
            Atom::Type(kind) => kind.span(),
            Atom::Map(map) => map.span(),
            Atom::Array(array) => array.span(),
            Atom::Tuple(tuple) => tuple.span(),
        }
    }
}
