use std::{
    collections::HashMap,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Clone)]
pub(crate) enum JSON {
    Null,
    String(String),
    MultiLine(String),
    Literal(String),
    Number(Number),
    Array(Vec<JSON>),
    Object(HashMap<String, JSON>),
}

#[derive(Clone)]
pub(crate) struct Number(pub String);
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
