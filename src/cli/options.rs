use std::path::PathBuf;

use crate::common::exit::exit;

use super::arguments::{Argument, Arguments};

pub struct BuildOptions {
    pub status: bool,
    pub project_dir: PathBuf,
}
impl From<Arguments> for BuildOptions {
    fn from(mut value: Arguments) -> Self {
        let mut options = Self {
            status: true,
            project_dir: value.current_dir().clone(),
        };
        while let Some(argument) = value.next_argument() {
            match argument {
                Argument::Value(value) => match value.as_str() {
                    "--disable-status" => options.status = false,
                    _ => exit(format!("No option found for: '{value}'")),
                },
                Argument::KeyValue(key, value) => match key.as_str() {
                    "--project-dir" => options.project_dir = PathBuf::from(value),
                    _ => exit(format!("No option found for key: '{value}'")),
                },
            }
        }

        if !options.project_dir.exists() {
            exit(format!(
                "Path to: '{:?}' doesn't exists",
                options.project_dir
            ));
        }

        options
    }
}
