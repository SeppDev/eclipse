use std::marker::PhantomData;

pub enum Value<K> {
    Reference(String, PhantomData<K>),
    Integer(String, PhantomData<K>),
    Float(String, PhantomData<K>),
    Boolean(bool, PhantomData<K>),
}
impl Value<Pointer> {
    pub fn new_reference(name: String) -> Self {
        Value::Reference(name, PhantomData)
    }
}
impl Value<Primitive> {
    pub fn new_integer(value: String) -> Self {
        Value::Integer(value, PhantomData)
    }
    pub fn new_float(value: String) -> Self {
        Value::Float(value, PhantomData)
    }
    pub fn new_bool(value: bool) -> Self {
        Value::Boolean(value, PhantomData)
    }
}

pub struct Pointer;
pub struct Primitive;