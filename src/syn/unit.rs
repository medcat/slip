use super::item::Item;
use super::{BasicNode, Node};
use diag::Span;
use error::*;
use stream::TokenStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit(Vec<Item>, Span);

impl Unit {
    pub fn items(&self) -> &[Item] {
        &self.0[..]
    }
    pub fn items_mut(&mut self) -> &mut [Item] {
        &mut self.0[..]
    }
}

impl Node for Unit {
    fn parse(stream: &mut TokenStream) -> Result<Unit> {
        let mut span = Span::identity();
        let mut items = vec![];
        while !stream.eof() {
            let item = Item::parse(stream)?;
            span |= item.span();
            items.push(item);
        }

        Ok(Unit(items, span))
    }
}

impl BasicNode for Unit {
    fn span(&self) -> Span {
        self.1
    }
}
