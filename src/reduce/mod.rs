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

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;
use std::collections::HashMap;
use std::sync::Arc;

mod annotation;
mod arecot;
mod process;
mod resolve;
mod type_;
#[macro_use]
mod path;

pub use self::annotation::{Annotation};
pub use self::path::Path;
pub use self::type_::TypeState;
use crate::diag::DiagnosticSync;

use crate::error::Error;
use crate::syn::Root;

pub struct Reduce<'s> {
    set: DiagnosticSync<'s>,
    context: Context,
    module: Module,
    annotated: HashMap<Path<'s>, Vec<Arc<Annotation<'s>>>>,
    types: HashMap<Path<'s>, BasicTypeEnum>,
    funcs: HashMap<Path<'s>, FunctionValue>,
}

impl<'s> Reduce<'s> {
    pub fn new(set: DiagnosticSync<'s>) -> Reduce<'s> {
        let context = Context::create();
        let module = context.create_module("mod");
        Reduce {
            set,
            context,
            module,
            annotated: HashMap::new(),
            types: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn push(&mut self, root: &'s Root) {
        for item in TypeState::build(root) {
            let annotation = Annotation::from(item);
            let name = annotation.to_path();
            self.annotated
                .entry(name)
                .or_insert_with(|| vec![])
                .push(Arc::new(annotation));
        }
    }

    pub fn reduce(&mut self) -> Result<(), Error> {
        process::verify_singluar_items(self);
        while let Some(name) = self.pluck() {
            self.process(name)?;
        }
        unimplemented!()
    }

    fn process(&mut self, name: Path<'s>) -> Result<(), Error> {
        let annotation = self
            .annotated
            .get(&name)
            .and_then(|t| t.first())
            .unwrap()
            .clone();
        if annotation.is_type() {
            process::kind(self, annotation)?;
        } else if annotation.is_func() {
            process::func(self, annotation)?;
        }

        Ok(())
    }

    fn pluck(&self) -> Option<Path<'s>> {
        self.annotated
            .iter()
            .find(|(key, value)| {
                let func_defined = self.funcs.contains_key(key);
                let type_defined = self.types.contains_key(key);
                let is_func = value
                    .first()
                    .map(AsRef::as_ref)
                    .map(Annotation::is_func)
                    .unwrap_or(false);
                let is_type = value
                    .first()
                    .map(AsRef::as_ref)
                    .map(Annotation::is_type)
                    .unwrap_or(false);

                !func_defined && !type_defined && (is_func || is_type)
            })
            .map(|s| s.0)
            .cloned()
    }
}
