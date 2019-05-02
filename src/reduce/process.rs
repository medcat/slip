use std::sync::Arc;

use inkwell::types::BasicType;
use inkwell::AddressSpace;

use super::resolve;
use super::{Annotation, Reduce};
use crate::diag::Name as DiagnosticName;
use crate::error::Error;
use crate::syn::{BasicNode, Item, Struct};

pub(super) fn kind<'s>(
    reduce: &mut Reduce<'s>,
    annotation: Arc<Annotation<'s>>,
) -> Result<(), Error> {
    match annotation.item() {
        Item::Struct(struct_) => build_struct(reduce, annotation, struct_),

        _ => unreachable!(),
    }

    unimplemented!()
}

fn build_struct<'s>(reduce: &mut Reduce<'s>, annotation: Arc<Annotation<'s>>, struct_: &'s Struct) {
    let items = struct_
        .elements()
        .iter()
        .map(|item| {
            resolve::kind(reduce, annotation.tstate(), item.kind())
                .unwrap_or_else(|| reduce.context.i64_type().as_basic_type_enum())
        })
        .collect::<Vec<_>>();
    let base = reduce
        .context
        .struct_type(&items[..], false)
        .ptr_type(AddressSpace::Generic);
}

pub(super) fn func<'s>(
    reduce: &mut Reduce<'s>,
    annotation: Arc<Annotation<'s>>,
) -> Result<(), Error> {
    unimplemented!()
}

pub(super) fn verify_singluar_items(reduce: &Reduce<'_>) {
    for (name, items) in reduce.annotated.iter().filter(|(_, i)| i.len() > 1) {
        let diagname = if name.fname().is_some() {
            DiagnosticName::FuncRedefinition
        } else {
            DiagnosticName::TypeRedefinition
        };
        reduce.set.emit(
            diagname,
            items.last().unwrap().span(),
            format!("item {} already defined", name),
        );
        for prev in items.iter().take(items.len() - 1) {
            reduce.set.emit_if(
                diagname,
                DiagnosticName::Note,
                prev.span(),
                "note: previous definition here",
            );
        }
    }
}
