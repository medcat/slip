use super::Item;
use super::{Build, State};
use crate::syn;
use crate::tacky::context;
use std::rc::Rc;

pub struct Struct<'a>(Vec<(&'a str, Rc<Item<'a>>)>);

impl<'a> Struct<'a> {
    fn build(from: &'a syn::Struct, build: &mut Build<'a>, state: &State<'a>) -> Struct<'a> {
        let items = from
            .elements()
            .iter()
            .map(|element| {
                (
                    element.value().value().unwrap(),
                    build.lookup_item(state, element.kind()),
                )
            })
            .collect::<Vec<(&'a str, Rc<Item<'a>>)>>();
        Struct(items)
    }
}
