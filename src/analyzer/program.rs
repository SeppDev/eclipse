use std::{collections::HashMap, task::Context};

use crate::{
    parser::{BaseType, Path, Type, Value},
    AnalyzeResult,
};

#[derive(Debug)]
pub struct Program {}

pub type Function = (Vec<(String, Type)>, Option<Type>);
pub type FunctionTypes = HashMap<Path, Function>;

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

pub struct Types {
    custom: HashMap<Path, CustomType>,
    pub generic_functions: HashMap<Path, Function>,
    pub functions: HashMap<Path, IRFunction>,
}
impl Types {
    pub fn get_type(&self, path: &Path) -> AnalyzeResult<&CustomType> {
        return match self.custom.get(path) {
            Some(t) => Ok(t),
            None => todo!(),
        };
    }
    pub fn push_struct(
        &mut self,
        path: Path,
        name: String,
        fields: Vec<(String, Type)>,
    ) -> AnalyzeResult<()> {
        self.custom
            .insert(path, CustomType::Struct(IRStruct { name, fields }));
        return Ok(());
    }
    pub fn push_enum(
        &mut self,
        path: Path,
        name: String,
        enums: Vec<(String, Option<Vec<Type>>)>,
    ) -> AnalyzeResult<()> {
        self.custom
            .insert(path, CustomType::Enum(IREnum { name, enums }));
        return Ok(());
    }
}

pub enum CustomType {
    Struct(IRStruct),
    Enum(IREnum),
}

pub struct IREnum {
    pub name: String,
    pub enums: Vec<(String, Option<Vec<Type>>)>,
}
pub struct IRStruct {
    pub name: String,
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
