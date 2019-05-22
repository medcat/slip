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
//! Each function takes a pointer to the arecot as the first argument,
//! and a pointer to the block of data as the second.

use crate::reduce::Path;
use crate::reduce::Reduce;
use crate::slip_path;
use inkwell::module::Linkage;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::FunctionValue;
use inkwell::AddressSpace;

static TYPE_ANNOT: Path<'static> = slip_path!(["$slip"]::["$arecot"]);
static INIT_ANNOT: Path<'static> = slip_path!(["$slip"]::["$arecot"].["$init"]);
static REF_ANNOT: Path<'static> = slip_path!(["$slip"]::["$arecot"].["$ref"]);
static DEREF_ANNOT: Path<'static> = slip_path!(["$slip"]::["$arecot"].["$deref"]);

pub(super) fn build(reduce: &mut Reduce<'_>) {
    reduce.types.insert(
        TYPE_ANNOT.clone(),
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
            .add_function(&INIT_ANNOT.to_string(), fn_type, Some(Linkage::External));
    reduce.funcs.insert(INIT_ANNOT.clone(), init_fn);
    let ref_fn =
        reduce
            .module
            .add_function(&REF_ANNOT.to_string(), fn_type, Some(Linkage::External));
    reduce.funcs.insert(REF_ANNOT.clone(), ref_fn);
    let deref_fn =
        reduce
            .module
            .add_function(&DEREF_ANNOT.to_string(), fn_type, Some(Linkage::External));
    reduce.funcs.insert(DEREF_ANNOT.clone(), deref_fn);
}

pub(super) fn type_from<'s>(reduce: &mut Reduce<'s>) -> BasicTypeEnum {
    *reduce.types.get(type_annot()).unwrap()
}

pub(super) fn type_annot() -> &'static Path<'static> {
    &TYPE_ANNOT
}

pub(super) fn init_from<'s>(reduce: &mut Reduce<'s>) -> FunctionValue {
    *reduce.funcs.get(init_annot()).unwrap()
}

pub(super) fn init_annot() -> &'static Path<'static> {
    &INIT_ANNOT
}

pub(super) fn ref_annot() -> &'static Path<'static> {
    &REF_ANNOT
}

pub(super) fn deref_annot() -> &'static Path<'static> {
    &DEREF_ANNOT
}
