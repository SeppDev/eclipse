use std::path::PathBuf;

use crate::common::exit::exit;

use super::arguments::{Argument, Arguments};

pub struct CommandLineOptions {
    pub status: bool,
    pub project_path: PathBuf,
}
impl From<Arguments> for CommandLineOptions {
    fn from(mut value: Arguments) -> Self {
        let mut options = Self {
            status: true,
            project_path: value.current_dir().clone(),
        };
        while let Some(argument) = value.next_argument() {
            match argument {
                Argument::Value(value) => match value.as_str() {
                    "--disable-status" => options.status = false,
                    _ => exit(format!("No option found for: '{value}'")),
                },
                Argument::KeyValue(key, value) => match key.as_str() {
                    "--project-dir" => options.project_path = PathBuf::from(value),
                    _ => exit(format!("No option found for key: '{key}'")),
                },
            }
        }

        if !options.project_path.exists() {
            exit(format!(
                "Path to: '{:?}' does not exists",
                options.project_path
            ));
        }

        options
    }
}
