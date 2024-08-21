//     match enum_type {
    // pub fn type_size(enum_type: &Type) -> usize {
//         Type::Tuple(types) => {
//             let mut total = 0;
//             for t in types {
//                 total += type_size(t);
//             }
//             return total
//         }
//         Type::String => 16,
//         Type::Integer(integer) => match integer {
//             Integer::u8 => 1,
//             Integer::i8 => 1,
//             Integer::u16 => 2,
//             Integer::i16 => 2,
//             Integer::u32 => 4,
//             Integer::i32 => 4,
//             Integer::u64 => 8,
//             Integer::i64 => 8,
//         }
//         // Type::Char => 4,
//     }
// }


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Integer {
    // u8,
    // i8,
    // u16,
    // i16,
    // u32,
    // i32,
    // u64,
    i64,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    body: Vec<Type>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    String,
    Boolean,
    Struct(String),
    Integer(Integer),
    // Tuple(Vec<Type>),
    // Char,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Division,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Value {
    Integer(isize),
    String(String),
    Boolean(bool)
}


#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Expression {
    Value(Value, Type),
    GetVariable(String),
    Call(String, Vec<Expression>),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>)
}



#[derive(Debug, Clone)]
pub enum Node {
    // EndOfFile,
    Scope(Vec<Node>),
    Call(String, Vec<Expression>),
    Return(Option<Expression>),
    Conditional((Expression, Expression), Vec<Node>, Option<Vec<Node>>),

    Module(String),
    // Include(),
    
    Function {
        public: bool,
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Option<Type>,
        body: Vec<Node>,
    },
    DefineVariable {
        name: String,
        mutable: bool,
        var_type: Option<Type>,
        expression: Option<Expression>,
    },
    SetVariable {
        name: String,
        expression: Expression
    },
}
