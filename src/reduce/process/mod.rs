use std::sync::Arc;

use super::resolve;
use super::{Annotation, Reduce};
use crate::diag::Name as DiagnosticName;
use crate::error::Error;
use crate::syn::{BasicNode, Item};

mod struct_;

pub(super) fn kind<'s>(
    reduce: &mut Reduce<'s>,
    annotation: Arc<Annotation<'s>>,
) -> Result<(), Error> {
    match annotation.item() {
        Item::Struct(struct_) => struct_::build(reduce, annotation, struct_),

        _ => unreachable!(),
    }

    unimplemented!()
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
