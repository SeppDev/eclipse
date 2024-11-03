use crate::compiler::types::Type;

#[derive(Debug)]
pub enum Node {
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Option<Type>,
        body: Vec<Node>
    }
}

#[derive(Debug)]
pub enum Expression {

}