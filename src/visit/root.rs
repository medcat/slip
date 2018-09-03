use syn::{Enum, Function, Item, Module, Struct, Use};

pub trait RootVisitor {
    type Error;

    fn visit_item(&mut self, item: &mut Item) -> Result<(), Self::Error> {
        match item {
            Item::Enum(enum_) => self.visit_enum(enum_),
            Item::Function(func) => self.visit_function(func),
            Item::Module(mod_) => self.visit_module(mod_),
            Item::Struct(struct_) => self.visit_struct(struct_),
            Item::Use(use_) => self.visit_use(use_),
        }
    }

    fn visit_enum(&mut self, _enum_: &mut Enum) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_struct(&mut self, _struct_: &mut Struct) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_use(&mut self, _use_: &mut Use) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_module(&mut self, mod_: &mut Module) -> Result<(), Self::Error> {
        for item in mod_.items_mut().iter_mut() {
            self.visit_item(item)?;
        }

        Ok(())
    }

    fn visit_function(&mut self, _func: &mut Function) -> Result<(), Self::Error> {
        Ok(())
    }
}
