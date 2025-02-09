use config::Config;
use status::Status;
use target::Target;

use crate::common::{
    arguments::Arguments,
    errors::{CompileError, CompileResult},
    files::{File, ProjectFiles},
};
use std::path::PathBuf;

pub mod config;
pub mod status;
pub mod target;

pub struct CompileCtx {
    pub target: Target,
    pub config: Config,
    project_files: ProjectFiles,
    status: Option<Status>,
}
impl CompileCtx {
    pub fn new(mut arguments: Arguments) -> CompileResult<Self> {
        let mut project_dir = arguments.current_dir().clone();
        let mut has_status = true;

        let target = Target::new();

        loop {
            let option = match arguments.next_option()? {
                Some(o) => o,
                None => break,
            };
            match option {
                CommandOption::ProjectPath(path) => project_dir = path,
                CommandOption::DisableStatus => has_status = false,
            }
        }

        let config = Config::open(&project_dir)?;
        let mut project_files = ProjectFiles::new(project_dir);
        project_files.pre_cache()?;

        Ok(Self {
            status: has_status.then(|| Status::new()),
            target,
            config,
            project_files,
        })
    }
    pub fn read(&self, relative_path: &PathBuf) -> CompileResult<Option<&File>> {
        self.project_files.read(relative_path)
    }
    pub fn message(&self, message: String) {
        if let Some(status) = &self.status {
            status.message(message)
        }
    }
    pub fn finish(self) {
        if let Some(status) = self.status {
            status.quit()
        }
    }
}

enum CommandOption {
    ProjectPath(PathBuf),
    DisableStatus,
}
impl Arguments {
    fn next_option(&mut self) -> CompileResult<Option<CommandOption>> {
        let string = match self.next() {
            Some(k) => k,
            None => return Ok(None),
        };

        match string.to_lowercase().as_str() {
            "--disable-status" => return Ok(Some(CommandOption::DisableStatus)),
            _ => {}
        }

        if let Some((key, value)) = string.split_once('=') {
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
        todo!()
    }
}
