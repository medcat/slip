use crate::syn;

mod pass;
mod type_state;

pub use self::type_state::TypeState;

pub struct Context<'s> {
    pub(crate) data: Vec<(TypeState<'s>, &'s syn::Item)>,
    pub(crate) func: Vec<(
        TypeState<'s>,
        &'s syn::function::FunctionName,
        &'s syn::Function,
    )>,
}

impl<'s> Context<'s> {
    pub fn new() -> Context<'s> {
        Context {
            data: vec![],
            func: vec![],
        }
    }

    pub fn pull(&mut self, root: &'s syn::Root) {
        let mut pass = pass::ContextPass::new();
        for item in root.items() {
            pass.visit(self, item);
        }
    }
}
