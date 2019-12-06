use super::{Name, Operation, TypeId};
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockId(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Name,
    pub generics: Vec<Name>,
    pub parameters: Vec<TypeId>,
    pub retval: Option<TypeId>,
    pub blocks: BTreeMap<BlockId, Vec<Operation>>,
}
