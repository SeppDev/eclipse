use std::marker::PhantomData;

use crate::json;

use super::{
    errors::{self, LSPResult},
    json::{ToJson, JSON},
};

use super::json::JSONObject;

impl ClientMessage {
    pub fn from_json(mut object: JSON) -> LSPResult<Self> {
        let method = match object
            .consume_result("method")?
            .as_string()
            .unwrap()
            .as_str()
        {
            "initialize" => Method::Initialize,
            m => return Err(Box::new(errors::UnkownMethod(m.to_string()))),
        };

        let id = object
            .consume_result("id")?
            .as_number()
            .unwrap()
            .as_usize()
            .unwrap();

        Ok(Self {
            marker: PhantomData,
            id,
            method,
        })
    }
}

pub type ClientMessage = Message<Client>;
pub type ServerMessage = Message<Server>;

pub struct Client;
pub struct Server;

pub struct Notification {
    pub method: Method,
}

pub struct Message<T> {
    marker: PhantomData<T>,
    pub id: usize,
    pub method: Method,
    // params:
}
impl ToJson for ServerMessage {
    fn to_json(self) -> JSONObject {
        json! {
            id:  self.id,
            method: self.method
        }
    }
}

pub enum Method {
    Initialize,
    Initialized,
}
impl ToJson for Method {
    fn to_json(self) -> JSONObject {
        return match self {
            Self::Initialize => json!("initialize"),
            Self::Initialized => json!("initialized"),
        };
    }
}
