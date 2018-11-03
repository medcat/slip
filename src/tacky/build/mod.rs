use crate::syn;
use crate::tacky::context::Context;
use std::collections::HashMap;

mod enum_;
mod func;
mod item;
mod struct_;

pub use self::func::Func;
pub use self::item::Item;

pub struct Build<'a> {
    context: &'a Context<'a>,
    items: HashMap<syn::Type, Item>,
    fns: HashMap<(syn::Type, syn::function::FunctionName), Func>,
}

impl<'a> Build<'a> {
    pub fn lookup_item(&mut self, kind: &'a syn::Type) -> &Item {
        match self.items.get(kind) {
            Some(value) => value,
            None => {
                let item = Item::build(kind, self.context);
                self.items.insert(kind.clone(), item);
                self.items.get(kind).unwrap()
            }
        }
    }
}
