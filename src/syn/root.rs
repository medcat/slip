use super::{BasicNode, Item, Node};
use crate::diag::Span;
use crate::error::*;
use crate::stream::TokenStream;

#[derive(Debug, Clone)]
pub struct Root(Vec<Item>, Span);

impl Root {
    pub fn items(&self) -> &[Item] {
        &self.0[..]
    }
    pub fn items_mut(&mut self) -> &mut [Item] {
        &mut self.0[..]
    }
}

impl Node for Root {
    fn parse(stream: &mut TokenStream) -> Result<Root> {
        let mut span = Span::identity();
        let mut items = vec![];

        while stream.peek().is_some() {
            let item = Item::parse(stream)?;
            span |= item.span();
            items.push(item);
        }

        Ok(Root(items, span))
    }
}

impl BasicNode for Root {
    fn span(&self) -> Span {
        self.1
    }
}
