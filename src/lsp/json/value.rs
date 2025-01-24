use std::{
    collections::HashMap,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Literal(String),
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Debug, Clone)]
pub struct Number(pub String);
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

impl Value {
    pub fn as_literal(self) -> Result<String, ()> {
        if let Value::Literal(string) = self {
            return Ok(string);
        } else {
            return Err(());
        }
    }
    pub fn as_string(self) -> Result<String, ()> {
        if let Value::String(string) = self {
            return Ok(string);
        } else {
            return Err(());
        }
    }
    pub fn as_number(self) -> Result<Number, ()> {
        if let Value::Number(number) = self {
            return Ok(number);
        } else {
            return Err(());
        }
    }
}

impl Value {
    pub fn insert(&mut self, key: String, value: Value) {
        if let Value::Object(map) = self {
            map.insert(key, value);
        } else {
            panic!()
        }
    }
    pub fn push(&mut self, value: Value) {
        if let Value::Array(array) = self {
            array.push(value);
        } else {
            panic!()
        }
    }
    pub fn rinsert(mut self, key: String, value: Value) -> Self {
        self.insert(key, value);
        return self;
    }
    pub fn rpush(mut self, value: Value) -> Self {
        self.push(value);
        return self;
    }

    pub fn consume(&mut self, key: &str) -> Option<Self> {
        if let Value::Object(map) = self {
            map.remove(key)
        } else {
            panic!()
        }
    }

    pub fn consume_result(&mut self, key: &str) -> Result<Self, String> {
        if let Value::Object(map) = self {
            match map.remove(key) {
                Some(v) => return Ok(v),
                None => return Err(format!("Failed to get value of '{key}'")),
            }
        } else {
            panic!()
        }
    }
}
