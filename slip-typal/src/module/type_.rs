use super::{Name, TypeReference};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Type {
    pub name: Name,
    pub generics: Vec<Name>,
    pub definition: TypeDefinition,
}

impl Type {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypeDefinition {
    Struct(HashMap<String, TypeReference>),
    Enum(Enum),
    Alias(TypeReference),
    Primitive(u64),
    PrimitiveSize,
    PrimitivePtr,
    Stub,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Enum {
    Simple(Vec<String>),
    Value(Vec<(String, ())>),
    Unit(Vec<(String, Vec<TypeReference>)>),
}
