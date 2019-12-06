use super::{BasicNode, Item, Node};
use crate::diag::Span;
use crate::error::*;
use crate::stream::TokenStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Root {
    items: Vec<Item>,
    area: Span,
}

impl Root {
    pub fn items(&self) -> &[Item] {
        &self.items[..]
    }
    pub fn items_mut(&mut self) -> &mut [Item] {
        &mut self.items[..]
    }
}

impl Node for Root {
    fn parse(stream: &mut TokenStream) -> Result<Root, Error> {
        let mut span = Span::identity();
        let mut items = vec![];

        while stream.peek().is_some() {
            let item = Item::parse(stream)?;
            span |= item.span();
            items.push(item);
        }

        Ok(Root { items, area: span })
    }
}

impl BasicNode for Root {
    fn span(&self) -> Span {
        self.area
    }
}
