use std::collections::HashMap;

use crate::{
    parser::{BaseType, Path, Type, Value},
    AnalyzeResult,
};

#[derive(Debug)]
pub struct Program {}

#[derive(Debug)]
pub struct IRFunction {
    pub generics: Option<Vec<String>>,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
}

#[derive(Debug)]
pub enum IRExpression {
    Value(Value),
    GetVariable(Path),
}

#[derive(Debug)]
pub enum IRNode {
    Return(Option<IRExpression>),
    // DefineVariable {
    //     name: String,
    //     data_type: Type,
    //     expression: Option<Expression>
    // }
}

#[derive(Debug, Default)]
pub struct Types {
    pub custom: HashMap<Path, CustomType>,
    pub generic_custom: HashMap<Path, CustomType>,

    pub generic_functions: HashMap<Path, IRFunction>,
    pub functions: HashMap<Path, IRFunction>,
}
impl Types {
    pub fn new() -> Self {
        return Self::default();
    }
    pub fn get_type(&self, path: &Path) -> AnalyzeResult<&CustomType> {
        return match self.custom.get(path) {
            Some(t) => Ok(t),
            None => todo!(),
        };
    }
}

#[derive(Debug)]
pub enum CustomType {
    Struct(IRStruct),
    Enum(IREnum),
}

#[derive(Debug)]
pub struct IREnum {
    pub name: String,
    pub generics: Option<Vec<String>>,
    pub enums: Vec<(String, Option<Vec<Type>>)>,
}

#[derive(Debug)]
pub struct IRStruct {
    pub name: String,
    pub generics: Option<Vec<String>>,
    pub fields: Vec<(String, Type)>,
    // pub implmentations
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IRType {
    Struct(String),
    Enum(String),
    Generic(String),
    Base(BaseType),
    Tuple(Vec<IRType>),
}
