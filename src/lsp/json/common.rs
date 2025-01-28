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
