use super::context::TypeState;
use super::Context;
use crate::diag::{Diagnostic, DiagnosticSet, Span};
use crate::error::Error;
use crate::syn;
use crate::syn::BasicNode;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, FunctionType};
use std::collections::HashMap;

mod build;
mod func;

pub struct Resolve<'m, 's> {
    module: &'m Module,
    ty: HashMap<&'s syn::Type, BasicTypeEnum>,
    fc: HashMap<String, (FunctionType, crate::diag::Span)>,
}

impl<'m, 's> Resolve<'m, 's> {
    pub fn new(module: &'m Module) -> Resolve<'m, 's> {
        Resolve {
            module,
            ty: HashMap::new(),
            fc: HashMap::new(),
        }
    }

    pub fn merge(
        &mut self,
        context: &Context<'s>,
        set: &mut DiagnosticSet<'_>,
    ) -> Result<(), Error> {
        let exported = context.func.iter().filter(|i| i.2.export());

        for export in exported {
            if export.0.base.generics().is_some() {
                generics_error(set, export.0.base.span())?;
            }

            if export.2.generics().len() > 0 {
                generics_error(set, export.2.span())?;
            }

            visit_func(self, context, &export.0, export.2, set)?;
        }

        Ok(())
    }
}

fn visit_func<'m, 's>(
    resolv: &mut Resolve<'m, 's>,
    context: &Context<'s>,
    tst: &TypeState<'s>,
    func: &'s syn::Function,
    set: &mut DiagnosticSet<'_>,
) -> Result<(), Error> {
    if tst.base.generics().is_some() {
        generics_error(set, tst.base.span());
    }

    if func.generics().len() > 0 {
        generics_error(set, func.generics().span());
    }

    let name = func_name(&tst.base, func.name().value());

    match resolv.fc.get(&name) {
        Some((_, v)) => {
            set.emit(
                Diagnostic::FuncRedefinition,
                func.span(),
                format!("function {} has already been defined", func.name().value()),
            )?;
            set.emit_if(
                Diagnostic::FuncRedefinition,
                Diagnostic::Note,
                *v,
                "function originally defined here".to_string(),
            )?;
            Ok(())
        }

        None => {
            unimplemented!()
            // let fc = func::of(resolv, context, tst, func, set)?;
            // resolv.fc.insert(name, (fc, func.span()));
            // Ok(())
        }
    }
}

fn func_name(ty: &syn::Type, name: &str) -> String {
    let iter = ty.parts().iter().flat_map(|t| t.value());
    let mut s = String::with_capacity(
        iter.clone()
            .map(|s| s.len() + 2)
            .fold(0, |acc, el| acc + el)
            + name.len()
            - 1,
    );
    let mut p = iter.peekable();
    while let Some(l) = p.next() {
        s += l;
        if p.peek().is_some() {
            s += "::";
        }
    }

    s += ".";
    s += name;

    s
}

fn generics_error(set: &mut DiagnosticSet<'_>, span: Span) -> Result<(), Error> {
    set.emit(
        Diagnostic::Generics,
        span,
        "generics are not currently supported".to_string(),
    )?;

    Ok(())
}
