use crate::diag::DiagnosticSet;
use crate::error::Error;
use crate::syn;
use std::collections::HashMap;

mod definition;
mod descent;
mod key;
mod kind;
mod name;

pub use self::definition::Definition;
pub use self::key::Key;
pub use self::kind::{FunctionId, UnconstrainedType};
pub use self::name::Name;

pub struct Reference {
    counter: usize,
    names: HashMap<Name<'static>, Key>,
    definition: HashMap<Key, Definition>,
}

impl Reference {
    pub fn new() -> Reference {
        Reference {
            counter: 0,
            names: HashMap::new(),
            definition: HashMap::new(),
        }
    }

    pub fn names_iter<'a>(&'a self) -> impl Iterator<Item = &'a Name<'static>> + 'a {
        self.names.keys()
    }

    pub fn lookup<'a>(&mut self, name: &'a Name<'a>) -> Key {
        match self.names.get(name) {
            Some(result) => *result,
            None => {
                let fixed = name.to_static();
                let current = self.counter;
                self.counter += 1;
                let key = Key(current);
                self.names.insert(fixed, key);
                key
            }
        }
    }

    pub fn contains<'a>(&self, name: &'a Name<'a>) -> bool {
        self.names.get(name).is_some()
    }

    pub fn define(&mut self, name: &Name, definition: Definition) -> Result<(), Error> {
        let key = self.lookup(name);
        match self.definition.get(&key) {
            Some(_) => unreachable!(), // error, for now
            None => {
                self.definition.insert(key, definition);
                Ok(())
            }
        }
    }

    pub fn descend(&mut self, top: &syn::Root, set: &mut DiagnosticSet) -> Result<(), Error> {
        let mut descent = self::descent::Descent::new(self, set);
        descent.descend(top)
    }
}
