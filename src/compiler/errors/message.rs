use crate::compiler::path::Path;

use super::Location;

#[derive(Debug, PartialEq)]
pub enum MessageKind {
    Note,
    Warning,
    Error,
}
impl std::fmt::Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub kind: MessageKind,
    pub message: String,
    pub details: Vec<Detail>,
}
impl Message {
    pub fn push<Notice: ToString>(&mut self, notice: Notice, location: Location) {
        self.details.push(Detail::new(notice.to_string(), location));
    }
}

#[derive(Debug, Clone)]
pub struct Detail {
    pub notice: String,
    pub location: Location,
}
impl Detail {
    pub fn new(notice: String, location: Location) -> Self {
        Self { notice, location }
    }
}