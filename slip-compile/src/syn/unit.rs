use super::item::Item;
use super::{BasicNode, Node};
use crate::diag::Span;
use crate::error::*;
use crate::stream::TokenStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    items: Vec<Item>,
    area: Span,
}

impl Unit {
    pub fn items(&self) -> &[Item] {
        &self.items[..]
    }
    pub fn items_mut(&mut self) -> &mut [Item] {
        &mut self.items[..]
    }
}

impl Node for Unit {
    fn parse(stream: &mut TokenStream) -> Result<Unit, Error> {
        let mut span = Span::identity();
        let mut items = vec![];
        while !stream.eof() {
            let item = Item::parse(stream)?;
            span |= item.span();
            items.push(item);
        }

        Ok(Unit { items, area: span })
    }
}

impl BasicNode for Unit {
    fn span(&self) -> Span {
        self.area
    }
}
