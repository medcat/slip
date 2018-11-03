use crate::syn;
use crate::tacky::context::Context;

pub enum Item {
    Empty,
}

impl Item {
    pub fn build<'a, 'b>(name: &'a syn::Type, context: &Context<'b>) -> Item {
        unimplemented!()
    }
}
