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

mod type_;
pub use self::type_::TypeState;

use crate::diag::DiagnosticSet;
use crate::syn::Root;

pub struct Reduce<'s, 't> {
    root: &'s Root,
    set: &'t mut DiagnosticSet,
}
