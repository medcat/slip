use super::{BasicNode, Item, Node, Type};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module(Type, Vec<Item>, Span);

impl Module {
    pub fn kind(&self) -> &Type {
        &self.0
    }

    pub fn kind_mut(&mut self) -> &mut Type {
        &mut self.0
    }

    pub fn items(&self) -> &[Item] {
        &self.1[..]
    }

    pub fn items_mut(&mut self) -> &mut [Item] {
        &mut self.1[..]
    }
}

impl Node for Module {
    fn parse(stream: &mut TokenStream) -> Result<Module, Error> {
        let mut span = stream.expect_one(TokenKind::Module)?.span();
        let kind = Type::parse(stream)?;
        span |= kind.span();
        span |= stream.expect_one(TokenKind::LeftBrace)?.span();

        let mut contents = vec![];

        while !stream.peek_one(TokenKind::RightBrace) {
            let item = Item::parse(stream)?;
            span |= item.span();
            contents.push(item);
        }

        span |= stream.expect_one(TokenKind::RightBrace)?.span();
        Ok(Module(kind, contents, span))
    }
}

impl BasicNode for Module {
    fn span(&self) -> Span {
        self.2
    }
}
