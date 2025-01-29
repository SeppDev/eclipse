use crate::json;

use super::{JSONObject, ToJson};

impl ToJson for Option<bool> {
    fn to_json(self) -> JSONObject {
        match self {
            Some(b) => json!(b),
            None => JSONObject::Null,
        }
    }
}

impl ToJson for char {
    fn to_json(self) -> JSONObject {
        json!(self.to_string())
    }
}

pub fn vec_to_json<T: ToJson>(object: Vec<T>) -> JSONObject {
    JSONObject::Array(object.into_iter().map(|f| f.to_json()).collect())
}
