use std::collections::{HashMap, HashSet};

use crate::compiler::{counter::NameCounter, errors::Location, nodes::hlir};

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
    future_keys: Vec<String>,
    counter: NameCounter,
    scopes: Vec<HashSet<String>>,
    keys: HashMap<String, String>,
}
impl Variables {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn generate(&mut self) -> String {
        let key = self.counter.increment();
        self.future_keys.push(key.clone());
        return key;
    }
    pub fn increment(&mut self) -> String {
        return self.counter.increment();
    }
    pub fn insert(
        &mut self,
        name: String,
        mutable: bool,
        data_type: hlir::Type,
        location: Location,
    ) -> Result<String, String> {
        // if name == "_" {
        //     return None;
        // }

        let key = match self.future_keys.pop() {
            Some(k) => k,
            None => self.increment(),
        };

        self.scopes.last_mut().unwrap().insert(name.clone());
        match self.keys.insert(name.clone(), key.clone()) {
            Some(k) => return Err(k),
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

        self.map.insert(key.clone(), variable);

        return Ok(key);
    }
    pub fn read(&mut self, name: &String) -> Option<&mut Variable> {
        let key = match self.keys.get(name) {
            Some(key) => key,
            None => return None,
        };
        return self.map.get_mut(key);
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
