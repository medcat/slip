use super::resolve;
use crate::error::Error;
use crate::reduce::{Annotation, Reduce};
use crate::syn::{Struct, Type};
use slip_typal::module::TypeDefinition;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) fn build<'s>(
    reduce: &mut Reduce<'s>,
    annotation: Arc<Annotation<'s>>,
    struct_: &'s Struct,
) -> Result<(), Error> {
    let name = annotation.to_path().to_name();
    let generics = annotation
        .generic_list()
        .map(Type::to_name)
        .collect::<Vec<_>>();
    let id = reduce.module.stub_type(name.clone(), generics.clone());

    let definitions = struct_
        .elements()
        .iter()
        .map(|el| {
            let name = el.value().value().map(str::to_string).unwrap();
            let tyid = resolve::kind(
                reduce,
                &annotation,
                el.kind(),
            );
            (name, tyid)
        })
        .collect::<HashMap<_, _>>();

    reduce.module.update_type(id, |type_| {
        type_.definition = TypeDefinition::Struct(definitions);
    });

    Ok(())
}
