use std::fmt;
use std::hash::{Hash, Hasher};

use crate::diag::Span;

use super::TypeState;
use crate::syn::function::FunctionName;
use crate::syn::{BasicNode, Item, Type};

/// An annotated item - an item, along with the path to the item (type path)
/// and the type imports.  This is easier to keep track of than having these
/// things be separate - plus, we generate the annotation name from this setup,
/// as well.
pub(super) struct Annotation<'s> {
    tstate: TypeState<'s>,
    item: &'s Item,
}

impl<'s> Annotation<'s> {
    pub(super) fn to_name(&self) -> AnnotationName<'s> {
        unimplemented!()
    }

    pub(super) fn is_type(&self) -> bool {
        match self.item {
            Item::Enum(_) | Item::Struct(_) => true,
            _ => false,
        }
    }

    pub(super) fn is_func(&self) -> bool {
        match self.item {
            Item::Function(_) => true,
            _ => false,
        }
    }

    pub(super) fn item(&self) -> &'s Item {
        self.item
    }

    pub(super) fn tstate(&self) -> &TypeState<'s> {
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

#[derive(Debug, Clone)]
/// The name associated with an annotation.  Since the actual type path may
/// vary in various references, we have to do a few things to handle this.
/// First, we say that `A::B` is a continuous path; next, if a module `C` is
/// defined within `A::B`, then we say that `[A::B, C]` is the completed path.
/// `[A::B, C]` is defined to be equivalent to any combination of continuous
/// paths, as long as the components are, in order, `A`, `B`, and `C`; in
/// words, `[A::B, C]` is equal to `[A, B::C]`, `[A::B::C]`, and `[A, B, C]`.
/// However, since they are all represented differently in terms of the type
/// structure, we store the completed path as an array here, and do a flat-map
/// on the parts to determine equality.
///
/// Note that this also takes into account function names, if the type
/// definition is a function.
pub(super) struct AnnotationName<'s> {
    type_: Vec<&'s Type>,
    fname: Option<&'s FunctionName>,
}

impl<'s> AnnotationName<'s> {
    pub(super) fn new(type_: Vec<&'s Type>, fname: Option<&'s FunctionName>) -> AnnotationName<'s> {
        AnnotationName { type_, fname }
    }

    pub(super) fn kind(&self) -> &[&'s Type] {
        &self.type_[..]
    }

    // we can do this because references implement copy.
    pub(super) fn fname(&self) -> Option<&'s FunctionName> {
        self.fname
    }
}

impl PartialEq for AnnotationName<'_> {
    fn eq(&self, other: &AnnotationName<'_>) -> bool {
        let self_parts = self.type_.iter().flat_map(|t| t.parts().iter());
        let other_parts = other.type_.iter().flat_map(|t| t.parts().iter());

        self_parts.eq(other_parts) && self.fname == other.fname
    }
}

impl Eq for AnnotationName<'_> {}

impl Hash for AnnotationName<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for part in self.type_.iter().flat_map(|t| t.parts().iter()) {
            part.hash(state);
        }

        self.fname.hash(state);
    }
}

impl fmt::Display for AnnotationName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let joined = Type::join_all(self.type_.iter().cloned());
        if let Some(fname) = self.fname {
            write!(f, "{}.{}", joined, fname.value())
        } else {
            joined.fmt(f)
        }
    }
}
