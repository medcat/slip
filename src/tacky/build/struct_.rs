use super::Build;
use super::Item;
use crate::syn;
use crate::tacky::context;

pub struct Struct(Vec<(String, Item)>);

fn build<'a>(
    from: &'a syn::Struct,
    state: &'a context::TypeState<'a>,
    context: &'a context::Context<'a>,
    build: &mut Build<'a>,
) -> Struct {
    let items = from
        .elements()
        .iter()
        .map(|element| {
            (
                element.value().value().unwrap().to_owned(),
                build.lookup(element.kind(), &state.uses()),
            )
        })
        .collect();

    Struct(items)
}
