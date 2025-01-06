pub enum Value {
    Reference(String),
    Integer(String),
    Float(String),
    Boolean(bool),
    Constant(Box<Value>)
}
impl Value {
    pub fn new_reference(name: String) -> Self {
        Value::Reference(name)
    }
    pub fn new_integer(value: String) -> Self {
        Value::Integer(value)
    }
    pub fn new_float(value: String) -> Self {
        Value::Float(value)
    }
    pub fn new_bool(value: bool) -> Self {
        Value::Boolean(value)
    }
}