use super::Context;
use super::TypeState;
use crate::syn;

pub(super) struct ContextPass<'s> {
    type_stack: Vec<&'s syn::Type>,
    use_stack: Vec<Vec<&'s syn::Use>>,
}

impl<'s> ContextPass<'s> {
    pub(super) fn new() -> ContextPass<'s> {
        ContextPass {
            type_stack: vec![],
            use_stack: vec![vec![]],
        }
    }

    fn build_state(&self) -> TypeState<'s> {
        let base = syn::Type::join_all(self.type_stack.iter().map(|v| *v));
        let uses = self.use_stack.iter().flatten().cloned().collect::<Vec<_>>();
        TypeState { base, uses }
    }

    pub(super) fn visit(&mut self, context: &mut Context<'s>, item: &'s syn::Item) {
        match item {
            syn::Item::Use(use_) => self.visit_use(context, use_),
            syn::Item::Function(func) => self.visit_function(context, func),
            syn::Item::Module(mod_) => self.visit_module(context, mod_),
            _ => {
                let state = self.build_state();
                context.data.push((state, item));
            }
        }
    }

    fn visit_use(&mut self, _context: &mut Context<'s>, use_: &'s syn::Use) {
        self.use_stack.last_mut().unwrap().push(use_);
    }

    fn visit_function(&mut self, context: &mut Context<'s>, func: &'s syn::Function) {
        let state = self.build_state();
        let name = func.name();

        context.func.push((state, name, func));
    }

    fn visit_module(&mut self, context: &mut Context<'s>, mod_: &'s syn::Module) {
        self.type_stack.push(mod_.kind());
        self.use_stack.push(vec![]);
        for item in mod_.items() {
            self.visit(context, item);
        }
        self.use_stack.pop();
        self.type_stack.pop();
    }
}
