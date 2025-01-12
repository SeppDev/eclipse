use std::collections::HashMap;

use crate::compiler::counter::NameCounter;

pub struct Variable {
    pub is_register_value: bool,
    pub key: String,
}

#[derive(Default)]
pub struct VariablesMap {
    future_keys: Vec<String>,
    counter: NameCounter,
    map: HashMap<String, Variable>, // Changed key to `String` to avoid lifetime issues
}

impl VariablesMap {
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
    pub fn insert(&mut self, name: String, is_register: bool) -> &str {
        let key = match self.future_keys.pop() {
            Some(k) => k,
            None => self.counter.increment(),
        };
        
        let old = self.map.insert(
            name.clone(),
            Variable {
                is_register_value: is_register,
                key: key.clone(),
            },
        );
        assert!(old.is_none());

        &self.map.get(&name).unwrap().key
    }
    pub fn get(&self, name: &String) -> &Variable {
        self.map.get(name).unwrap()
    }
}
