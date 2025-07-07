use common::exit::exit;

use super::arguments::{Argument, Arguments};
use std::path::PathBuf;

pub struct CommandLineOptions {
    pub status: bool,
    pub release: bool,
    pub active_path: PathBuf,
}
impl From<Arguments> for CommandLineOptions {
    fn from(mut value: Arguments) -> Self {
        let mut options = Self {
            status: true,
            release: false,
            active_path: value.current_dir().to_owned(),
        };

        while let Some(argument) = value.next_argument() {
            match argument {
                Argument::Value(value) => match value.as_str() {
                    "--disable-status" => options.status = false,
                    "--release" => options.release = true,
                    _ => exit(format!("No option found for: '{value}'")),
                },
                Argument::KeyValue(key, value) => match key.as_str() {
                    "--project-dir" => options.active_path = PathBuf::from(value),
                    _ => exit(format!("No option found for key: '{key}'")),
                },
            }
        }

        if !options.active_path.exists() {
            exit(format!(
                "Path to: '{}' does not exists",
                options.active_path.to_string_lossy()
            ));
        }

        options
    }
}
