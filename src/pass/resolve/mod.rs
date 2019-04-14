use super::context::TypeState;
use super::Context;
use crate::diag::{Diagnostic, DiagnosticSet, Span};
use crate::error::Error;
use crate::syn;
use crate::syn::BasicNode;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

mod func;
mod name;
pub use self::name::Name;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(usize);

pub struct Resolve<'s> {
    look: HashMap<Name<'s>, ItemId>,
    count: AtomicUsize,
}

impl<'s> Resolve<'s> {
    pub fn new() -> Resolve<'s> {
        Resolve {
            items: HashMap::new(),
            look: HashMap::new(),
            count: AtomicUsize::new(0),
        }
    }

    pub fn try_from(
        context: &Context<'s>,
        set: &mut DiagnosticSet<'_>,
    ) -> Result<Resolve<'s>, Error> {
        let exported = context.func.iter().filter(|i| i.2.export());
        let mut resolve = Resolve::new();

        for export in exported {
            if export.0.base.generics().is_some() {
                generics_error(set, export.0.base.span());
            }

            if export.2.generics().len() > 0 {
                generics_error(set, export.2.span());
            }

            visit_func(&mut resolve, context, &export.0, export.2, set)?;
        }

        Ok(resolve)
    }
}

fn visit_func<'s>(
    resolv: &mut Resolve<'s>,
    context: &Context<'s>,
    tst: &TypeState<'s>,
    func: &'s syn::Function,
    set: &mut DiagnosticSet<'_>,
) -> Result<ItemId, Error> {
    let name = Name::from_func(func, &tst.base);
    if let Some(item) = resolv.look.get(&name) {
        return Ok(*item);
    }

    let item = ItemId(resolv.count.fetch_add(1, Ordering::SeqCst));
    resolv.look.insert(name, item);
}

fn generics_error(set: &mut DiagnosticSet<'_>, span: Span) {
    set.emit(
        Diagnostic::Generics,
        span,
        "generics are not currently supported".to_string(),
    );
}
