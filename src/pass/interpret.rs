use super::context::Context;
use crate::error::*;
use crate::syn;

struct Interpreter<'s>(Context<'s>);

impl<'s> Interpreter<'s> {
    pub fn run_main(&self) -> Result<()> {
        let func = self.0.func.iter().find(|f| {
            f.0.base.parts().len() == 0 && match f.1 {
                syn::function::FunctionName::Identifier(ident) if ident.value() == Some("main") => {
                    true
                }
                _ => false,
            }
        });

        func.ok_or_else(|| Error::from(ErrorKind::MissingMainError))
            .and_then(|v| self.call(v.2))
    }

    fn call(&self, func: &syn::Function) -> Result<()> {
        unimplemented!()
    }
}
