use super::enum_::Enum;
use super::function::Function;
use super::module::Module;
use super::struct_::Struct;
use super::use_::Use;
use super::{BasicNode, Node};
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Module(Module),
    Use(Use),
}

impl Node for Item {
    fn parse(stream: &mut TokenStream) -> Result<Item> {
        match stream.peek_kind() {
            Some(TokenKind::Fn) => Ok(Item::Function(Function::parse(stream)?)),
            Some(TokenKind::Struct) => Ok(Item::Struct(Struct::parse(stream)?)),
            Some(TokenKind::Enum) => Ok(Item::Enum(Enum::parse(stream)?)),
            Some(TokenKind::Module) => Ok(Item::Module(Module::parse(stream)?)),
            Some(TokenKind::Use) => Ok(Item::Use(Use::parse(stream)?)),
            _ => stream
                .error_from(&[
                    TokenKind::Fn,
                    TokenKind::Struct,
                    TokenKind::Enum,
                    TokenKind::Module,
                    TokenKind::Use,
                ]).map(|_| unreachable!()),
        }
    }
}

impl BasicNode for Item {
    fn span(&self) -> Span {
        match self {
            Item::Function(func) => func.span(),
            Item::Struct(struct_) => struct_.span(),
            Item::Enum(enum_) => enum_.span(),
            Item::Module(module) => module.span(),
            Item::Use(use_) => use_.span(),
        }
    }
}
