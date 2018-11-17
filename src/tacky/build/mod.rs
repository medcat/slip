use crate::syn;
use crate::tacky::context::{Context, TypeState};
use std::collections::HashMap;
use std::rc::Rc;

mod enum_;
mod func;
mod item;
mod struct_;

pub use self::func::Func;
pub use self::item::Item;

pub struct Build<'a> {
    context: &'a Context<'a>,
    items: HashMap<syn::Type, Rc<Item<'a>>>,
    fns: HashMap<(syn::Type, syn::function::FunctionName), Func>,
}

impl<'a> Build<'a> {
    pub fn lookup_item(
        &mut self,
        state: &State<'a>,
        tstate: &TypeState<'a>,
        kind: &syn::Type,
    ) -> Rc<Item<'a>> {
        match self.items.get(kind) {
            Some(value) => value.clone(),
            None => {
                let item = Rc::new(Item::build(self, state, kind));
                self.items.insert(kind.clone(), item.clone());
                item.clone()
            }
        }
    }
}

pub struct State<'a>(HashMap<&'a syn::Type, &'a syn::Type>);

impl<'a> State<'a> {
    fn lookup(
        &self,
        kind: &'a syn::Type,
        tstate: &TypeState<'a>,
        context: &'a Context<'a>,
    ) -> Option<(Item<'a>, State<'a>)> {
    }
}
