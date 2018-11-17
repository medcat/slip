use inkwell::context::Context;
use inkwell::module::Module;

pub(crate) mod context;
mod reference;

pub fn build(name: &str, context: &context::Context<'_>) {
    let con = Context::create();
    let mod_ = con.create_module(name);
}
