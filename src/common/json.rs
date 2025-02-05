use std::{
    collections::BTreeMap,
    num::{ParseFloatError, ParseIntError},
};

pub mod from;

#[derive(Debug, Clone)]
pub(crate) enum JSON {
    Null,
    String(String),
    MultiLine(String),
    Literal(String),
    Number(Number),
    Array(Vec<JSON>),
    Object(BTreeMap<String, JSON>),
}
impl JSON {
    pub fn new() -> JSON {
        JSON::Object(BTreeMap::new())
    }
    pub fn array() -> JSON {
        JSON::Array(Vec::new())
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
    pub fn as_string(self) -> String {
        self.0
    }
}
