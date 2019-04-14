//! This is probably the most complicated part of the parser; as such,
//! I think it needs some explaination.  At the "core" of an expression,
//! there is an "Atom."  An atom is an unambiguous complete expression -
//! such as a number, a string, an identifier, etc.  After that, it gets
//! more complicated.  Let's take the expression `1 + 2 * 4`.  `1`, `2`,
//! and `4` are all "atoms."  This expression, however, is ambiguous,
//! because we can end up with two interpretations: `(1 + 2) * 4`, or
//! `1 + (2 * 4)`.  Our definition of order for these operations says
//! that the latter is correct - but the parser has to correctly
//! represent that in its tree.  This is how it does it:
//!
//! First, it consumes the atom `1`.  It then sees the `+`.  Since there
//! is no symbol to compare against, for precedence (the "default
//! precedence"), it consumes it.  It then recurses, in order to parse
//! a new expression.  The parser consumes the `2` atom.  It then sees
//! the `*`.  The previous symbol was `+`, which has a lower precedence
//! than `*`.  So it consumes the `*`, and recurses to parse a new
//! expression.  It then consumes the `2`.  Since it no longer sees any
//! tokens that can follow an expression, it returns up the stack, which
//! produces a tree that is effectively `1 + (2 * 4)`.
//!
//! This relies on a concept of "precedence" that is associated with
//! these tokens.  Separate tokens have different precedence, and can
//! be either left- or right-associative - this is important in cases
//! where you have two tokens with the same precedence; left-associative
//! means that it'll return, right-associative means it'll consume the
//! next token.

use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node};
use serde_derive::*;

mod access;
mod array;
mod atom;
mod call;
mod index;
mod infix;
mod map;
mod precedence;
mod prefix;
mod suffix;
mod tuple;

pub use self::access::Access;
pub use self::array::Array;
pub use self::atom::Atom;
pub use self::call::Call;
pub use self::index::Index;
pub use self::infix::InfixOperation;
pub use self::map::{Map, MapPair};
use self::precedence::Precedence;
pub use self::prefix::PrefixOperation;
pub use self::suffix::SuffixOperation;
pub use self::tuple::Tuple;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An expression.  This is the result of parsing a (sometimes)
/// ambiguous recursive expression, which must yield a value.
pub enum Expression {
    Infix(InfixOperation),
    Suffix(SuffixOperation),
    Prefix(PrefixOperation),
    Call(Call),
    Access(Access),
    Index(Index),
    Atom(Atom),
}

impl Expression {
    fn parse_prec(stream: &mut TokenStream, prec: Precedence) -> Result<Expression, Error> {
        let mut base = Expression::parse_atom(stream)?;
        let mut next = stream.peek_kind();
        while prec.stay(next.into()) {
            base = match next {
                Some(TokenKind::Period) => Expression::Access(Access::parse(stream, base)?),
                Some(TokenKind::LeftParen) => Expression::Call(Call::parse(stream, base)?),
                Some(TokenKind::LeftBrace) => Expression::Index(Index::parse(stream, base)?),
                Some(TokenKind::DoublePlus) | Some(TokenKind::DoubleMinus) => {
                    Expression::Suffix(SuffixOperation::parse(stream, base)?)
                }
                Some(TokenKind::LessThan)
                | Some(TokenKind::LessThanEqual)
                | Some(TokenKind::GreaterThan)
                | Some(TokenKind::GreaterThanEqual)
                | Some(TokenKind::Compare)
                | Some(TokenKind::Plus)
                | Some(TokenKind::Minus)
                | Some(TokenKind::Star)
                | Some(TokenKind::Divide)
                | Some(TokenKind::LeftShift)
                | Some(TokenKind::RightShift)
                | Some(TokenKind::BitwiseAnd)
                | Some(TokenKind::BitwiseOr)
                | Some(TokenKind::BitwiseXor)
                | Some(TokenKind::LogicalAnd)
                | Some(TokenKind::LogicalOr)
                | Some(TokenKind::NotEqual)
                | Some(TokenKind::Equals)
                | Some(TokenKind::Modulo) => {
                    Expression::Infix(InfixOperation::parse(stream, base)?)
                }
                _ => unreachable!(),
            };

            next = stream.peek_kind();
        }

        Ok(base)
    }

    fn parse_atom(stream: &mut TokenStream) -> Result<Expression, Error> {
        match stream.peek_kind() {
            Some(TokenKind::DoublePlus)
            | Some(TokenKind::DoubleMinus)
            | Some(TokenKind::Plus)
            | Some(TokenKind::Minus)
            | Some(TokenKind::LogicalNot)
            | Some(TokenKind::BitwiseNot) => {
                Ok(Expression::Prefix(PrefixOperation::parse(stream)?))
            }
            _ => Ok(Expression::Atom(Atom::parse(stream)?)),
        }
    }
}

impl Node for Expression {
    fn parse(stream: &mut TokenStream) -> Result<Expression, Error> {
        Expression::parse_prec(stream, Precedence::Default)
    }
}

impl BasicNode for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Infix(infix) => infix.span(),
            Expression::Suffix(suffix) => suffix.span(),
            Expression::Prefix(prefix) => prefix.span(),
            Expression::Call(call) => call.span(),
            Expression::Access(access) => access.span(),
            Expression::Index(index) => index.span(),
            Expression::Atom(atom) => atom.span(),
        }
    }
}
