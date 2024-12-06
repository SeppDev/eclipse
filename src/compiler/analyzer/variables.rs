use std::collections::{HashMap, HashSet};

use crate::compiler::{counter::NameCounter, errors::Location, types::Type};

#[derive(Debug, Clone)]
pub struct Variable {
    pub mutable: bool,
    pub data_type: Type,
    pub key: String,
    pub location: Location,
    pub is_parameter: bool,
}

#[derive(Debug)]
pub struct VariablesMap {
    counter: NameCounter,
    states: Vec<HashSet<String>>,
    variables: HashMap<String, Variable>,
}
impl VariablesMap {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            variables: HashMap::new(),
            counter: NameCounter::new(),
        }
    }
    pub fn increment(&mut self) -> String {
        self.counter.increment()
    }

    pub fn insert(
        &mut self,
        is_parameter: bool,
        name: &String,
        mutable: bool,
        data_type: Type,
        location: Location,
    ) -> &Variable {
        let key = self.increment();
        let current_state = self.states.last_mut().unwrap();
        
        let variable = Variable {
            key,
            mutable,
            data_type,
            location,
            is_parameter,
        };

        let _ = self.variables.insert(name.clone(), variable);

        current_state.insert(name.clone());

        return self.read(&name).unwrap();
    }
    pub fn push_scope(&mut self) {
        self.states.push(HashSet::new());
    }
    pub fn pop_scope(&mut self) {
        let state = self.states.pop().unwrap();
        for key in state {
            self.variables.remove(&key);
        }
    }
    pub fn read(&self, name: &String) -> Option<&Variable> {
        return self.variables.get(name);
    }
    // pub fn write(&mut self, key: &String) -> bool {
    //     match self.variables.get_mut(key) {
    //         Some(var) => {
    //             var.mutated = true;
    //         }
    //         None => return false,
    //     };
    //     return true;
    // }
    // pub fn read(&mut self, key: &String) -> bool {
    //     match self.variables.get_mut(key) {
    //         Some(var) => {
    //             var.read = true;
    //         }
    //         None => return false,
    //     };
    //     return true;
    // }
}

// pub fn create_missing_message<'a>(debug: &'a mut CompileCtx, location: Location, name: &String) -> &'a mut Message {
//     return debug.error(location, format!("Could not find variabled named: '{name}'"))
// }
