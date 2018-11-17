use super::struct_::Struct;
use super::{Build, State};
use crate::syn;
use crate::tacky::context::Context;
use std::marker::PhantomData;

pub enum Item<'a> {
    Empty(PhantomData<&'a u8>),
    Struct(Struct<'a>),
}

impl<'a> Item<'a> {
    pub fn build(build: &mut Build<'a>, state: &State<'a>, name: &syn::Type) -> Item<'a> {
        unimplemented!()
    }
}
