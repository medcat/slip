//! Reduces the current active AST into an LLVM module.  This can be said to be
//! the actual interpreter for the language.  This has two primary components
//! to it - the core library, which takes an LLVM module and adds the "core"
//! components (like the integer types, void type, bool type, etc.) - and the
//! processing of the AST into the llvm module.  The end result is an llvm
//! module that can be either compiled AOT or JIT.
//!
//! The stages of the processing happen like this:
//!
//! 1. Step through all "items" in an AST, annotating the AST with a typestate.
//!    This typestate contains the current path to the item, i.e. the module
//!    definitions, as well as the current "uses," in order to be able to
//!    resolve type references.
//! 2. Build up type definitions on the items.  Start with a random item and
//!    trace its components down until all items defined have been converted
//!    into a type definition, marking up the missing or unknown types (and
//!    replacing them with a void type or unsized type).
//! 3. Build up function definitions.  These will be exported to define the
//!    behavior of the overall module.  Ideally such a file should contain a
//!    "main" function, which allows it to interoperate with the system.

use std::collections::HashMap;
use std::sync::Arc;

mod annotation;
mod type_;

use self::annotation::{Annotation, AnnotationName, AnnotationNameSlice};
pub use self::type_::TypeState;
use crate::diag::{Diagnostics, Name as DiagnosticName};

use crate::error::Error;
use crate::syn::{BasicNode, Root, Item, Type};

pub struct Reduce<'s> {
    set: Arc<Diagnostics>,
    annotated: HashMap<Arc<AnnotationName<'s>>, Vec<Arc<Annotation<'s>>>>,
    types: HashMap<Arc<AnnotationName<'s>>, ()>,
    funcs: HashMap<Arc<AnnotationName<'s>>, ()>,
}

impl<'s> Reduce<'s> {
    pub fn new(set: Arc<Diagnostics>) -> Reduce<'s> {
        Reduce {
            set,
            annotated: HashMap::new(),
            types: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn push(&mut self, root: &'s Root) {
        for item in TypeState::build(root) {
            let annotation = Annotation::from(item);
            let name = annotation.to_name();
            self.annotated
                .entry(Arc::new(name))
                .or_insert_with(|| vec![])
                .push(Arc::new(annotation));
        }
    }

    pub fn reduce(&mut self) -> Result<(), Error> {
        verify_singluar_items(self);
        while let Some(name) = self.pluck() {
            self.process(name)?;
        }
        unimplemented!()
    }

    fn process(&mut self, name: Arc<AnnotationName<'s>>) -> Result<(), Error> {
        let annotation = self
            .annotated
            .get(name.as_ref())
            .and_then(|t| t.first())
            .unwrap()
            .clone();
        if annotation.is_type() {
            process_type(self, annotation)?;
        } else if annotation.is_func() {
            process_func(self, annotation)?;
        }

        Ok(())
    }

    fn pluck(&self) -> Option<Arc<AnnotationName<'s>>> {
        self.annotated
            .iter()
            .find(|(key, value)| {
                let func_defined = self.funcs.contains_key(key.as_ref());
                let type_defined = self.types.contains_key(key.as_ref());
                let is_func = value.first().map(AsRef::as_ref).map(Annotation::is_func).unwrap_or(false);
                let is_type = value.first().map(AsRef::as_ref).map(Annotation::is_type).unwrap_or(false);

                !func_defined && !type_defined && (is_func || is_type)
            })
            .map(|s| s.0)
            .cloned()
    }
}

fn process_type<'s>(reduce: &mut Reduce<'s>, annotation: Arc<Annotation<'s>>) -> Result<(), Error> {
    match annotation.item() {
        Item::Struct(struct_) => {
            let items = struct_.elements().iter().map(|item| {
                resolve(reduce, annotation.tstate(), item.kind())
            });
        }

        _ => unreachable!()
    }

    unimplemented!()
}

fn process_func<'s>(reduce: &mut Reduce<'s>, annotation: Arc<Annotation<'s>>) -> Result<(), Error> {
    unimplemented!()
}

fn resolve<'s>(reduce: &mut Reduce<'s>, tstate: &TypeState<'s>, kind: &'s Type) -> Result<(), Error> {
    let applicable = 
        tstate.uses()
            .iter()
            .flat_map(|use_| use_.trails().iter().map(move |trail| (use_, trail)))
            .filter(|(_, trail)| trail.applies(kind))
            .map(|(use_, trail)| {
                trail.combine(use_.prefix(), kind)
            })
            .flat_map(|typ| {
                let anno = AnnotationNameSlice::new(&[&typ], None);
                reduce.annotated.get_key_value(&anno)
            });
    unimplemented!()
}

fn verify_singluar_items(reduce: &Reduce<'_>) {
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