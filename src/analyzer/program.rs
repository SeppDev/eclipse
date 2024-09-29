use crate::parser::{BaseType, Path, Type, Value};

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

#[derive(Debug)]
pub enum CustomType {
    Struct(IRStruct),
    Enum(IREnum),
}

#[derive(Debug)]
pub struct IREnum {
    pub name: String,
    pub generics: Option<Vec<String>>,
    pub enums: Vec<(String, Option<Type>)>,
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
