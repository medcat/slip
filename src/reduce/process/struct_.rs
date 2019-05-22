use super::resolve;
use crate::reduce::{arecot, Reduce, Path, Annotation};
use crate::syn::Struct;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValue, FunctionValue};
use inkwell::AddressSpace;
use std::sync::Arc;

pub(super) fn build<'s>(
    reduce: &mut Reduce<'s>,
    annotation: Arc<Annotation<'s>>,
    struct_: &'s Struct,
) {
    let name = annotation.to_path();
    let items = create_item_list(reduce, &annotation, struct_);
    let (base, combine) = create_type(reduce, &items);
    let init_fn = define_base_create(reduce, &name, base);
    define_new(reduce, &name, base, combine, init_fn);
}

fn create_item_list<'s>(
    reduce: &mut Reduce<'s>,
    annotation: &Annotation<'s>,
    struct_: &'s Struct,
) -> Vec<BasicTypeEnum> {
    struct_
        .elements()
        .iter()
        .map(|item| {
            resolve::kind(reduce, annotation.tstate(), item.kind())
                .unwrap_or_else(|| reduce.context.i64_type().as_basic_type_enum())
        })
        .collect::<Vec<_>>()
}

fn create_type<'s>(reduce: &mut Reduce<'s>, items: &[BasicTypeEnum]) -> (StructType, StructType) {
    let struct_ = reduce.context.struct_type(&items[..], false);
    let arecot = arecot::type_from(reduce);
    let combine = reduce
        .context
        .struct_type(&[arecot, struct_.as_basic_type_enum()], true);
    (struct_, combine)
}

fn define_base_create<'s>(
    reduce: &mut Reduce<'s>,
    name: &Path<'s>,
    base: StructType,
) -> FunctionValue {
    let null_fn =
        reduce
            .module
            .add_function(&format!("{}.$null", name), base.fn_type(&[], false), None);
    let builder = reduce.context.create_builder();
    let basic_block = reduce.context.append_basic_block(&null_fn, "entry");
    builder.position_at_end(&basic_block);
    builder.build_return(Some(&base.const_null()));

    let init_fn = reduce.module.add_function(
        &format!("{}.$init", name),
        base.fn_type(&base.get_field_types(), false),
        None,
    );
    let builder = reduce.context.create_builder();
    let basic_block = reduce.context.append_basic_block(&init_fn, "entry");
    builder.position_at_end(&basic_block);
    let point = builder.build_alloca(base, "base");
    for (i, param) in init_fn.get_param_iter().enumerate() {
        let gep = unsafe { builder.build_struct_gep(point, i as u32, "param") };
        builder.build_store(gep, param);
    }

    let loaded = builder.build_load(point, "load");
    builder.build_return(Some(&loaded));

    reduce
        .funcs
        .insert(name.clone().with_fname(Some("$null")), null_fn);
    reduce
        .funcs
        .insert(name.clone().with_fname(Some("$init")), init_fn);
    init_fn
}

fn define_new<'s>(
    reduce: &mut Reduce<'s>,
    name: &Path<'s>,
    base: StructType,
    combine: StructType,
    init_fn: FunctionValue,
) {
    let ret = combine
        .ptr_type(AddressSpace::Generic)
        .fn_type(&base.get_field_types(), false);
    let new_fn = reduce
        .module
        .add_function(&format!("{}.$new", name), ret, None);
    let builder = reduce.context.create_builder();
    let basic_block = reduce.context.append_basic_block(&new_fn, "entry");
    builder.position_at_end(&basic_block);

    let arecot_init_fn = arecot::init_from(reduce);

    let value = builder.build_malloc(combine, "base");
    let arecot_value = unsafe { builder.build_struct_gep(value, 0, "arecot") };
    let struct_value = unsafe { builder.build_struct_gep(value, 1, "struct") };
    builder.build_call(
        arecot_init_fn,
        &[
            arecot_value.as_basic_value_enum(),
            value.as_basic_value_enum(),
        ],
        "_arecot_init",
    );

    let init_value = builder
        .build_call(init_fn, &new_fn.get_params(), "init_value")
        .try_as_basic_value()
        .left()
        .unwrap();
    builder.build_store(struct_value, init_value);
    builder.build_return(Some(&value));

    reduce
        .funcs
        .insert(name.clone().with_fname(Some("$new")), new_fn);
    reduce
        .funcs
        .insert(name.clone().with_fname(Some("new")), new_fn);
}
