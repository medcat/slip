use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::values::FunctionValue;

pub(super) struct Build<'f>(&'f mut FunctionValue, &'f Context, BasicBlock);

impl<'f> Build<'f> {
    pub(super) fn new(fv: &'f mut FunctionValue, con: &'f Context) -> Build<'f> {
        let bb = con.append_basic_block(fv, "entry");
        Build(fv, con, bb)
    }
}
