use std::{
    collections::BTreeMap,
    num::{ParseFloatError, ParseIntError},
};

pub mod from;

type Object = BTreeMap<String, JSON>;

#[derive(Debug, Clone)]
pub(crate) enum JSON {
    Null,
    Boolean(bool),
    String(String),
    MultiLine(String),
    Literal(String),
    Number(Number),
    Array(Vec<JSON>),
    Object(Object),
}
impl JSON {
    pub fn new() -> JSON {
        JSON::Object(Object::new())
    }
    pub fn array() -> JSON {
        JSON::Array(Vec::new())
    }
}

impl JSON {
    pub fn as_mut_object(&mut self) -> &mut Object {
        if let Self::Object(object) = self {
            object
        } else {
            unreachable!()
        }
    }
}

impl JSON {
    pub fn insert<T: ToString>(&mut self, key: T, value: Self) -> Option<JSON> {
        let object = self.as_mut_object();
        object.insert(key.to_string(), value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Number(pub(super) String);
impl Number {
    pub fn as_isize(&self) -> Result<isize, ParseIntError> {
        self.0.parse()
    }
    pub fn as_usize(&self) -> Result<usize, ParseIntError> {
        self.0.parse()
    }
    pub fn as_f64(&self) -> Result<f64, ParseFloatError> {
        self.0.parse()
    }
    pub fn as_f32(&self) -> Result<f32, ParseFloatError> {
        self.0.parse()
    }
    pub fn as_string(&self) -> &String {
        &self.0
    }
}
