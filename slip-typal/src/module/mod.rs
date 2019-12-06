mod funcs;
mod name;
mod operation;
mod type_;

pub use self::funcs::*;
pub use self::name::Name;
pub use self::operation::Operation;
pub use self::type_::*;
use crate::version::Version;
use std::collections::BTreeMap;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct TypeId(u64);

impl TypeId {
    fn next(&self) -> TypeId {
        TypeId(self.0 + 1)
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct FunctionId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeReference {
    #[serde(rename = "abs")]
    Absolute(TypeId),
    #[serde(rename = "gen")]
    Generic(u64),
    #[serde(rename = "mix")]
    Mix(TypeId, Vec<TypeReference>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub version: Version,
    pub requirements: Vec<Requirement>,
    pub types: BTreeMap<TypeId, Type>,
    pub funcs: BTreeMap<FunctionId, Function>,
}

impl Module {
    pub fn next_type_id(&self) -> TypeId {
        self.types
            .range(..)
            .next_back()
            .map(|(i, _)| i.next())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub name: String,
    pub version: Version,
}
