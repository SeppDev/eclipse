use std::collections::{HashMap, HashSet};

use crate::compiler::{errors::Location, nodes::hlir};

#[derive(Debug, Default)]
pub struct Variable {
    pub name: String,
    pub key: String,
    pub location: Location,
    pub mutable: bool,
    pub data_type: hlir::Type,

    pub modified: bool,
    pub used: bool,
}

#[derive(Debug, Default)]
pub struct Variables {
    pub map: HashMap<String, Variable>,
    scopes: Vec<HashSet<String>>,
    keys: HashMap<String, String>,
}
impl Variables {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(
        &mut self,
        name: String,
        key: String,
        mutable: bool,
        data_type: hlir::Type,
        location: Location,
    ) -> Option<String> {
        // if name == "_" {
        //     return None;
        // }

        self.scopes.last_mut().unwrap().insert(name.clone());
        match self.keys.insert(name.clone(), key.clone()) {
            Some(k) => return Some(k),
            None => {}
        }

        let variable = Variable {
            name,
            key: key.clone(),
            location,
            mutable,
            data_type,

            modified: false,
            used: false,
        };

        self.map.insert(key, variable);

        return None;
    }
    pub fn read(&mut self, name: &String) -> Option<&Variable> {
        let key = match self.keys.get(name) {
            Some(key) => key,
            None => return None,
        };
        return match self.map.get_mut(key) {
            Some(var) => {
                var.used = true;
                Some(var)
            }
            None => None,
        };
    }
    pub fn get(&self, name: &String) -> Option<&Variable> {
        let key = match self.keys.get(name) {
            Some(key) => key,
            None => return None,
        };
        return match self.map.get(key) {
            Some(var) => Some(var),
            None => None,
        };
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }
    pub fn pop_scope(&mut self) {
        let state = self.scopes.pop().unwrap();
        for name in state {
            self.keys.remove(&name).unwrap();
        }
    }
}
