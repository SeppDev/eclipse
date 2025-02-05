use std::{error::Error, fmt::Display, path::PathBuf};

pub type CompileResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum CompileError {
    Unkown,
    PathNotFound(PathBuf),
    MissingConfig,
}
impl CompileError {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unkown => "Unkown",
            Self::PathNotFound(_) => "Path not found",
            Self::MissingConfig => "Mising config file",
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PathNotFound(path) => format!(
                    "Path does not exist: '{}', ",
                    path.to_str().unwrap().to_string()
                ),
                _ => self.as_str().to_string(),
            }
        )
    }
}

impl Error for CompileError {
    fn description(&self) -> &str {
        self.as_str()
    }
}
