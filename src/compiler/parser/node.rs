use crate::compiler::types::Type;

#[derive(Debug)]
pub enum Node {
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Node>,
    },
    Variable {
        name: String,
        value: Expression
    },
    Return(Option<Expression>)
}

pub struct NodeInfo {
    pub node: Node,
    
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(String)
}

#[derive(Debug)]
pub enum Value {
    Integer { minus: bool, integer: String },
}
