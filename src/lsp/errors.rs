use std::error::Error;
use std::fmt::Display;

pub type LSPResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct UnkownMethod(pub String);
impl Display for UnkownMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unkown method: '{}'", &self.0)
    }
}
impl Error for UnkownMethod {}
