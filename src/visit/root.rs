use super::Tail;
use crate::syn::{Enum, Function, Item, Module, Struct, Use};

pub trait RootVisitor<'s> {
    type Error;

    fn visit_item(&mut self, item: &'s Item) -> Tail<(), Self::Error> {
        match item {
            Item::Enum(enum_) => self.visit_enum(enum_),
            Item::Function(func) => self.visit_function(func),
            Item::Module(mod_) => self.visit_module(mod_),
            Item::Struct(struct_) => self.visit_struct(struct_),
            Item::Use(use_) => self.visit_use(use_),
        }
    }

    fn visit_enum(&mut self, _enum_: &'s Enum) -> Tail<(), Self::Error> {
        Ok(None)
    }

    fn visit_struct(&mut self, _struct_: &'s Struct) -> Tail<(), Self::Error> {
        Ok(None)
    }

    fn visit_use(&mut self, _use_: &'s Use) -> Tail<(), Self::Error> {
        Ok(None)
    }

    fn visit_module(&mut self, mod_: &'s Module) -> Tail<(), Self::Error> {
        for item in mod_.items() {
            self.visit_item(item)?;
        }

        Ok(None)
    }

    fn visit_function(&mut self, _func: &'s Function) -> Tail<(), Self::Error> {
        Ok(None)
    }
}
