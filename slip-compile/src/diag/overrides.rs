use super::{Level, Name};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub(super) struct Overrides {
    overrides: Vec<HashMap<Name, Level>>,
    cache: HashMap<Name, Level>,
}

impl Overrides {
    pub fn new() -> Overrides {
        Overrides {
            overrides: vec![HashMap::new()],
            cache: HashMap::new(),
        }
    }

    pub fn push(&mut self) {
        self.overrides.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        if self.overrides.len() == 1 {
            let empty = self.overrides[0].is_empty();
            self.overrides[0].clear();
            if !empty {
                self.rebuild();
            }
        } else if let Some(result) = self.overrides.pop() {
            if !result.is_empty() {
                self.rebuild();
            }
        }
    }

    pub fn insert(&mut self, diag: Name, level: Level) {
        self.overrides.last_mut().unwrap().insert(diag, level);
    }

    pub fn lookup(&self, diag: Name) -> Level {
        *self.cache.get(&diag).unwrap_or(&diag.level())
    }

    pub fn rebuild(&mut self) {
        self.cache = self.overrides.iter().fold(HashMap::new(), |mut acc, val| {
            acc.extend(val.iter());
            acc
        })
    }
}
