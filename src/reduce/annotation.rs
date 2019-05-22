use super::TypeState;
use crate::diag::Span;
use crate::reduce::Path;
use crate::syn::{BasicNode, Item};

/// An annotated item - an item, along with the path to the item (type path)
/// and the type imports.  This is easier to keep track of than having these
/// things be separate - plus, we generate the annotation name from this setup,
/// as well.
pub struct Annotation<'s> {
    tstate: TypeState<'s>,
    item: &'s Item,
}

impl<'s> Annotation<'s> {
    pub fn to_path(&self) -> Path<'s> {
        let mut items = self.tstate.base().to_owned();
        let mut fname = None;
        match self.item {
            Item::Enum(enum_) => items.push(enum_.kind()),
            Item::Struct(struct_) => items.push(struct_.kind()),
            Item::Function(func_) => fname = Some(func_.name()),
            _ => {}
        }
        Path::from_syn(items, fname)
    }

    pub fn is_type(&self) -> bool {
        match self.item {
            Item::Enum(_) | Item::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_func(&self) -> bool {
        match self.item {
            Item::Function(_) => true,
            _ => false,
        }
    }

    pub fn item(&self) -> &'s Item {
        self.item
    }

    pub fn tstate(&self) -> &TypeState<'s> {
        &self.tstate
    }
}

impl<'s> From<(TypeState<'s>, &'s Item)> for Annotation<'s> {
    fn from((tstate, item): (TypeState<'s>, &'s Item)) -> Annotation<'s> {
        Annotation { tstate, item }
    }
}

impl BasicNode for Annotation<'_> {
    fn span(&self) -> Span {
        self.item.span()
    }
}
