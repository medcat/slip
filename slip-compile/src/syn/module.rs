use super::{BasicNode, Item, Node, Type};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    name: Type,
    items: Vec<Item>,
    area: Span,
}

impl Module {
    pub fn kind(&self) -> &Type {
        &self.name
    }

    pub fn kind_mut(&mut self) -> &mut Type {
        &mut self.name
    }

    pub fn items(&self) -> &[Item] {
        &self.items[..]
    }

    pub fn items_mut(&mut self) -> &mut [Item] {
        &mut self.items[..]
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
        Ok(Module {
            name: kind,
            items: contents,
            area: span,
        })
    }
}

impl BasicNode for Module {
    fn span(&self) -> Span {
        self.area
    }
}
