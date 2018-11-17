use crate::tacky::reference::Key;
use serde_derive::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "v")]
pub enum GenericKey {
    Generic(usize),
    Key(Key),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Type(GenericKey, Vec<Type>);

impl Type {
    pub fn new(base: GenericKey, generics: Vec<Type>) -> Type {
        Type(base, generics)
    }

    pub fn base(&self) -> GenericKey {
        self.0
    }
}
