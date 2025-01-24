use std::marker::PhantomData;

use super::{
    errors::{self, LSPResult},
    json::JSON,
};

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
            sender: PhantomData,
            id,
            method,
        })
    }
}

type ClientMessage = Message<Client>;
type ServerMessage = Message<Server>;

pub struct Client;
pub struct Server;

pub struct Message<T> {
    pub(super) sender: PhantomData<T>,
    pub id: usize,
    pub method: Method,
    // params:
}

pub enum Method {
    Initialize,
}
