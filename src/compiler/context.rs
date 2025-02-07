use config::Config;
use status::Status;
use target::Target;

use crate::common::{
    arguments::Arguments,
    errors::{CompileError, CompileResult},
};
use std::path::PathBuf;

pub mod config;
pub mod status;
pub mod target;

pub struct CompileCtx {
    pub status: Status,
    pub target: Target,
    pub config: Config,
}
impl CompileCtx {
    pub fn new(mut arguments: Arguments) -> CompileResult<Self> {
        let mut project_dir = arguments.current_dir().clone();
        let target = Target::new();

        loop {
            let option = match arguments.next_option()? {
                Some(o) => o,
                None => break,
            };
            match option {
                CommandOption::ProjectPath(path) => project_dir = path,
            }
        }

        let config = match Config::open(project_dir) {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        };

        Ok(Self {
            status: Status::new(),
            target,
            config,
        })
    }
}

enum CommandOption {
    ProjectPath(PathBuf),
}
impl Arguments {
    fn next_option(&mut self) -> CompileResult<Option<CommandOption>> {
        let (key, value) = match self.next() {
            Some(string) => match string.split_once('=') {
                Some((key, value)) => (key.trim().to_string(), value.to_string()),
                None => panic!("Failed to parse option"),
            },
            None => return Ok(None),
        };

        match key.to_lowercase().as_str() {
            "--project_path" => {
                let path = PathBuf::from(value);
                if path.exists() {
                    return Ok(Some(CommandOption::ProjectPath(path)));
                }
                return Err(Box::new(CompileError::PathNotFound(path)));
            }
            _ => todo!(),
        }
    }
}
