


#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<(String, IRType)>,
    pub return_type: IRType,
    pub body: Vec<Operation>,
}

#[derive(Debug)]
pub enum Operation {
    Label(String),
    Allocate(String, IRType),
    Return(IRType, IRValue),
}

#[derive(Debug)]
pub enum IRValue {
    IntLiteral(String),
    FloatLiteral(String),
    Variable(String),
    Null
}


#[derive(Debug)]
pub enum IRType {
    Tuple(Vec<IRType>),
    Pointer(Box<IRType>),
    Integer(usize),
    Array(usize, Box<IRType>),
    Float,
    Double,
    Void,
}
impl std::fmt::Display for IRType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IRType::Void => "void".to_string(),
                IRType::Double => "double".to_string(),
                IRType::Float => "float".to_string(),
                IRType::Array(size, t) => format!("[ {} x {} ]", size, t),
                IRType::Integer(bits) => format!("i{}", bits),
                IRType::Pointer(t) => format!("*{}", t),
                IRType::Tuple(types) => format!("{{ {} }}", {
                    let mut strings = Vec::new();
                    for t in types {
                        strings.push(format!("{}", t))
                    }
                    strings.join(", ")
                }),
            }
        )
    }
}
