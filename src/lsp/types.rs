use super::json::{JSONObject, ToJson, JSON};
use error_codes::ErrorCode;
mod error_codes;
mod initialize;
pub use initialize::*;

pub enum ValidOrNull<T> {
    Value(T),
    Null,
}

pub struct URI(String);
pub type DocumentURI = URI;

pub enum Params {
    Any(JSON),
}

pub enum StringOrInt {
    String(String),
    Integer(isize),
}

pub enum LSPAny {
    JSON(JSON),
    String(String),
    Integer(isize),
    UInteger(usize),
    Float(f32),
    Boolean(bool),
}

#[derive(Default)]
pub enum MessageId {
    #[default]
    Null,
    String(String),
    Integer(isize),
}

#[derive(Default)]
pub struct RequestMessage {
    pub id: MessageId,
    pub method: String,
    pub params: Option<Params>,
}

#[derive(Default)]
pub struct NotificationMessage {
    pub method: String,
    pub params: Option<Params>,
}

pub enum Response {
    Message {
        id: MessageId,
        result: Option<LSPAny>,
        error: Option<ErrorCode>,
    },
    Error {
        code: ErrorCode,
        message: String,
        data: Option<LSPAny>,
    },
}
// impl ToJson for Response {
//     fn to_json(self) -> JSONObject {
//         match self {
//             Self::Message { id, result, error } => json! {
//                 id: id,
//                 result: result,
//                 error: error
//             },
//             Self::Error {
//                 code,
//                 message,
//                 data,
//             } => json! {
//                 code: code,
//                 message: message,
//                 data: data
//             },
//         }
//     }
// }

pub type ProgressToken = StringOrInt;

pub struct ProgressParams<T> {
    token: ProgressToken,
    value: T,
}
