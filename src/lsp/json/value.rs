use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

use crate::lsp::types::FailureHandlingKind;

use super::ToJson;

#[derive(Debug, Clone)]
pub(crate) enum JSONObject {
    Null,
    Literal(String),
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<JSONObject>),
    Object(HashMap<String, JSONObject>),
}
impl ToJson for JSONObject {
    fn to_json(self) -> JSONObject {
        JSONObject::Null
    }
}
impl Display for JSONObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Null => "null".to_string(),
                Self::Number(n) => n.to_string(),
                Self::Literal(x) | Self::String(x) => x.to_string(),
                Self::Bool(x) => x.to_string(),
                Self::Array(vec) => vec.iter().map(|x| x.to_string()).collect::<String>(),
                Self::Object(map) => format!(
                    "{{ {} }}",
                    map.iter()
                        .map(|(k, v)| format!("{k}: {v}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
    }
}

impl From<isize> for JSONObject {
    fn from(value: isize) -> Self {
        Self::Number(Number::from(value))
    }
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
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        Self(value.to_string())
    }
}

impl JSONObject {
    pub fn as_literal(self) -> Result<String, ()> {
        if let JSONObject::Literal(string) = self {
            return Ok(string);
        } else {
            return Err(());
        }
    }
    pub fn as_string(self) -> Result<String, ()> {
        if let JSONObject::String(string) = self {
            return Ok(string);
        } else {
            return Err(());
        }
    }
    pub fn as_number(self) -> Result<Number, ()> {
        if let JSONObject::Number(number) = self {
            return Ok(number);
        } else {
            return Err(());
        }
    }
}

impl JSONObject {
    pub fn insert(&mut self, key: String, value: JSONObject) {
        if let JSONObject::Object(map) = self {
            map.insert(key, value);
        } else {
            panic!()
        }
    }
    pub fn insert_option<T: ToJson>(&mut self, key: String, value: Option<T>) {
        let object = match value {
            Some(t) => t.to_json(),
            None => JSONObject::Null,
        };
        self.insert(key, object)
    }
    pub fn push_option<T: ToJson>(&mut self, value: Option<T>) {
        let object = match value {
            Some(t) => t.to_json(),
            None => JSONObject::Null,
        };
        self.push(object)
    }

    pub fn push(&mut self, value: Self) {
        if let JSONObject::Array(array) = self {
            array.push(value);
        } else {
            panic!()
        }
    }
    pub fn rpush<Object: ToJson>(mut self, value: Object) -> Self {
        self.push(value.to_json());
        self
    }
    pub fn rinsert<T: ToString, Object: ToJson>(mut self, key: T, value: Object) -> Self {
        self.insert(key.to_string(), value.to_json());
        self
    }
    pub fn rinsert_option<T: ToString, Object: ToJson>(
        mut self,
        key: T,
        value: Option<Object>,
    ) -> Self {
        match value {
            Some(val) => self.insert(key.to_string(), val.to_json()),
            None => {}
        }
        self
    }
    pub fn rpush_option<Object: ToJson>(mut self, value: Option<Object>) -> Self {
        match value {
            Some(val) => self.push(val.to_json()),
            None => {}
        }
        self
    }
    pub fn consume(&mut self, key: &str) -> Option<Self> {
        if let JSONObject::Object(map) = self {
            map.remove(key)
        } else {
            panic!()
        }
    }
    pub fn consume_result(&mut self, key: &str) -> Result<Self, JSONError> {
        if let JSONObject::Object(map) = self {
            match map.remove(key) {
                Some(v) => return Ok(v),
                None => {
                    return Err(JSONError::FailedToGetKey {
                        key: key.to_string(),
                    })
                }
            }
        } else {
            panic!()
        }
    }
    pub fn new() -> Self {
        Self::Object(HashMap::new())
    }
}

#[derive(Debug)]
pub enum JSONError {
    FailedToGetKey { key: String },
}

impl Display for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FailedToGetKey { key } => format!("Failed to get value of '{key}'"),
            }
        )
    }
}

impl Error for JSONError {
    fn description(&self) -> &str {
        match self {
            Self::FailedToGetKey { .. } => "Failed to get value",
        }
    }
}
