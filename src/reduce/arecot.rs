//! Arecot = Atomic REference COunT.  We use this instead of a proper GC.
//! An arecot type is placed at the beginning of any block of data that is
//! reference counted.  When the data is referenced, the Arecot should be
//! incremented, and when it is unreferenced, the Arecot should be decremented.
//! When the reference count is zero, it should be removed.  Thus, we define
//! three functions:
//!
//! - `$slip::$arecot.init(void*, void*)`
//! - `$slip::$arecot.ref(void*, void*)`
//! - `$slip::$arecot.deref(void*, void*)`
//!
//! Each function takes a pointer to the block of data as the first argument,
//! and a pointer to the arecot as the second.

use inkwell::module::Linkage;
use inkwell::types::BasicType;
use inkwell::AddressSpace;
use lazy_static::*;
use std::sync::Arc;

use super::AnnotationName;
use super::Reduce;
use crate::syn::function::FunctionName;
use crate::syn::Type;

lazy_static! {
    static ref ARECOT_TYPE: Type = Type::from(["$slip", "$arecot"].iter().cloned());
    static ref INIT_NAME: FunctionName = FunctionName::ident_of("init");
    static ref REF_NAME: FunctionName = FunctionName::ident_of("ref");
    static ref DEREF_NAME: FunctionName = FunctionName::ident_of("deref");
    static ref TYPE_ANNOT: AnnotationName<'static> = AnnotationName::new(vec![&*ARECOT_TYPE], None);
    static ref INIT_ANNOT: AnnotationName<'static> =
        AnnotationName::new(vec![&*ARECOT_TYPE], Some(&*INIT_NAME));
    static ref REF_ANNOT: AnnotationName<'static> =
        AnnotationName::new(vec![&*ARECOT_TYPE], Some(&*REF_NAME));
    static ref DEREF_ANNOT: AnnotationName<'static> =
        AnnotationName::new(vec![&*ARECOT_TYPE], Some(&*DEREF_NAME));
}

pub(super) fn build(reduce: &mut Reduce<'_>) {
    reduce.types.insert(
        Arc::new(AnnotationName::new(vec![&*ARECOT_TYPE], None)),
        reduce
            .context
            .void_type()
            .ptr_type(AddressSpace::Generic)
            .as_basic_type_enum(),
    );
    let void_ptr = reduce
        .context
        .void_type()
        .ptr_type(AddressSpace::Generic)
        .as_basic_type_enum();
    let fn_type = reduce
        .context
        .void_type()
        .fn_type(&[void_ptr, void_ptr], false);
    let init_fn =
        reduce
            .module
            .add_function("$slip::$arecot.init", fn_type, Some(Linkage::External));
    reduce.funcs.insert(
        Arc::new(AnnotationName::new(vec![&*ARECOT_TYPE], Some(&*INIT_NAME))),
        init_fn,
    );
    let ref_fn = reduce
        .module
        .add_function("$slip::$arecot.ref", fn_type, Some(Linkage::External));
    reduce.funcs.insert(
        Arc::new(AnnotationName::new(vec![&*ARECOT_TYPE], Some(&*REF_NAME))),
        ref_fn,
    );
    let deref_fn =
        reduce
            .module
            .add_function("$slip::$arecot.deref", fn_type, Some(Linkage::External));
    reduce.funcs.insert(
        Arc::new(AnnotationName::new(vec![&*ARECOT_TYPE], Some(&*DEREF_NAME))),
        deref_fn,
    );
}

pub(super) fn type_annot() -> &'static AnnotationName<'static> {
    &*TYPE_ANNOT
}

pub(super) fn init_annot() -> &'static AnnotationName<'static> {
    &*INIT_ANNOT
}

pub(super) fn ref_annot() -> &'static AnnotationName<'static> {
    &*REF_ANNOT
}

pub(super) fn deref_annot() -> &'static AnnotationName<'static> {
    &*DEREF_ANNOT
}
