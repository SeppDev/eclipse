use std::collections::HashMap;

use crate::{
    parser::{Path, Type, Value},
    AnalyzeResult, CompileError,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Program {}

#[allow(unused)]
#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    GetVariable(Path),
}

#[allow(unused)]
#[derive(Debug)]
pub enum IRNode {
    Return(Option<IRExpression>),
    Scope { is_unsafe: bool, body: Vec<IRNode> }, // DefineVariable {
                                                  //     name: String,
                                                  //     data_type: Type,
                                                  //     expression: Option<Expression>
                                                  // }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<IRNode>,
}

#[derive(Debug)]
pub struct Variable {
    pub mutable: bool,
    pub data_type: Option<Type>,
}

#[derive(Debug)]
pub struct Variables {
    states: Vec<Vec<String>>,
    variables: HashMap<String, Variable>,
}
impl Variables {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            variables: HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        key: String,
        mutable: bool,
        data_type: Option<Type>,
    ) -> AnalyzeResult<()> {
        let current_state = self.states.last_mut().unwrap();
        current_state.push(key.clone());

        self.variables
            .insert(key.clone(), Variable { mutable, data_type });

        return Ok(());
    }
    pub fn create_state(&mut self) {
        self.states.push(Vec::new());
    }
    pub fn pop_state(&mut self) {
        let state = self.states.pop().unwrap();
        for key in state {
            self.variables.remove(&key);
        }
    }
    pub fn get(&self, key: &String) -> AnalyzeResult<&Variable> {
        return match self.variables.get(key) {
            Some(var) => Ok(var),
            None => return Err(CompileError::new(format!("{:?} is not defined", key), 0)),
        };
    }
    pub fn change_type(&mut self, key: &String, new_type: Type) -> AnalyzeResult<()> {
        let variable = self.variables.get_mut(key).unwrap();
        variable.data_type = Some(new_type);

        return Ok(());
    }
}
