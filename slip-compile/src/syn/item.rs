use super::enum_::Enum;
use super::function::Function;
use super::module::Module;
use super::struct_::Struct;
use super::use_::Use;
use super::{BasicNode, Node};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{Type, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Item {
    Function(Box<Function>),
    Struct(Box<Struct>),
    Enum(Box<Enum>),
    Module(Box<Module>),
    Use(Box<Use>),
}

impl Item {
    pub fn kind(&self) -> Option<&Type> {
        match self {
            Item::Struct(struct_) => Some(struct_.kind()),
            Item::Enum(enum_) => Some(enum_.kind()),
            _ => None
        }
    }

    pub fn generics(&self) -> Option<&Roll<Type>> {
        match self {
            Item::Struct(struct_) => struct_.kind().generics().as_ref(),
            Item::Enum(enum_) => enum_.kind().generics().as_ref(),
            Item::Function(func) => Some(func.generics()),
            Item::Module(mod_) => mod_.kind().generics().as_ref(),
            _ => None
        }
    }
}

impl Node for Item {
    fn parse(stream: &mut TokenStream) -> Result<Item, Error> {
        match stream.peek_kind() {
            Some(TokenKind::Fn) => Ok(Item::Function(Box::new(Function::parse(stream)?))),
            Some(TokenKind::Struct) => Ok(Item::Struct(Box::new(Struct::parse(stream)?))),
            Some(TokenKind::Enum) => Ok(Item::Enum(Box::new(Enum::parse(stream)?))),
            Some(TokenKind::Module) => Ok(Item::Module(Box::new(Module::parse(stream)?))),
            Some(TokenKind::Use) => Ok(Item::Use(Box::new(Use::parse(stream)?))),
            _ => stream
                .error_from(&[
                    TokenKind::Fn,
                    TokenKind::Struct,
                    TokenKind::Enum,
                    TokenKind::Module,
                    TokenKind::Use,
                ])
                .map(|_| unreachable!()),
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
