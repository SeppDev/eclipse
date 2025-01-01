use std::collections::{HashMap, HashSet};

use crate::compiler::{
    codegen::Operations,
    counter::NameCounter,
    errors::{CompileResult, Location},
    path::Path,
    types::Type,
};

#[derive(Debug)]
pub struct LoopInfo {
    pub begin: String,
    pub end: String,
}
impl LoopInfo {
    pub fn new<T: ToString, E: ToString>(begin: T, end: E) -> Self {
        Self {
            begin: begin.to_string(),
            end: end.to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Variable {
    pub mutable: bool,
    pub data_type: Type,
    pub key: String,
    pub location: Location,
}

#[derive(Debug)]
pub struct FunctionCtx<'a> {
    pub return_type: Option<Type>,
    pub operations: &'a mut Operations,
    pub relative_file_path: &'a Path,
    pub loop_info: Vec<LoopInfo>,

    counter: NameCounter,
    variable_scopes: Vec<HashSet<String>>,
    variables: HashMap<String, Variable>,
}
impl<'a> FunctionCtx<'a> {
    pub fn new(
        return_type: Option<Type>,
        operations: &'a mut Operations,
        relative_file_path: &'a Path,
    ) -> Self {
        Self {
            return_type,
            operations,
            relative_file_path,
            loop_info: Vec::new(),
            counter: NameCounter::new(),
            variable_scopes: Vec::new(),
            variables: HashMap::new(),
        }
    }
    pub fn pop_vars_scope(&mut self) {
        let state = self.variable_scopes.pop().unwrap();
        for key in state {
            self.variables.remove(&key);
        }
    }
    pub fn push_vars_scope(&mut self) {
        self.variable_scopes.push(HashSet::new());
    }
    pub fn increment_key(&mut self) -> String {
        return self.counter.increment();
    }
    pub fn insert_variable(
        &mut self,
        name: String,
        key: Option<String>,
        mutable: bool,
        data_type: Type,
        location: Location,
    ) -> CompileResult<()> {
        let key = match key {
            Some(k) => k,
            None => self.increment_key(),
        };

        if name == "_".to_string() {
            return Ok(());
        }

        let _ = self
            .variable_scopes
            .last_mut()
            .unwrap()
            .insert(name.clone());

        if self.variables.get(&name).is_some() {
            return Err(());
        }

        self.variables.insert(
            name,
            Variable {
                mutable,
                data_type,
                key,
                location,
            },
        );

        return Ok(());
    }
    pub fn read_variable(&self, name: &String) -> Option<&Variable> {
        return self.variables.get(name);
    }
}
