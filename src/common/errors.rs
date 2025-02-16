use std::error::Error;
pub type CompileError<T> = Result<T, Box<dyn Error>>;