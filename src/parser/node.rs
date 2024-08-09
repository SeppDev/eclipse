#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // u8,
    // i8,
    // u16,
    // i16,
    // u32,
    // i32,
    // u64,
    i64,
    // Char,
}
pub fn type_size(enum_type: &Type) -> usize {
    match enum_type {
        Type::i64 => 8,
    }
}

// #[derive(Debug, Clone)]
// pub enum Operator {
//     Plus,
//     Minus,
//     Multiply,
//     Division,
// }

#[derive(Debug, Clone)]
pub enum Expression {
    Value(isize),
    GetVariable(String),
    // BinaryOperation(isize, Operator, isize)
}

#[derive(Debug, Clone)]
pub enum Node {
    // EndOfFile,
    Scope(Vec<Node>),
    Call(String, Vec<Expression>),
    // Conditional(Vec<(Expression, Expression)>, Vec<Node>, Option<Vec<Node>>),
    Conditional((Expression, Expression), Vec<Node>, Option<Vec<Node>>),
    
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        return_types: Option<Vec<Type>>,
        body: Vec<Node>,
    },
    DefineVariable {
        name: String,
        mutable: bool,
        var_type: Type,
        expression: Option<Expression>,
    },
}
