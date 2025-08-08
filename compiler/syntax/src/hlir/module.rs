use super::{Node, Parameter, Type};

#[derive(Debug)]
pub struct Module {
    pub imports: Vec<String>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Node>,
}


#[derive(Debug, Default)]
pub struct ModuleCollection {
    pub modules: Vec<Module>,
}
