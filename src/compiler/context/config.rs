use std::path::PathBuf;

use crate::common::{errors::CompileResult, toml::TOML};

pub const CONFIG_NAME: &str = "eclipse";

pub struct Config {
    pub name: String,
    pub version: String,
}

impl Config {
    pub fn open(project_dir: &PathBuf) -> CompileResult<Self> {
        let mut config_path = project_dir.join(CONFIG_NAME);
        config_path.set_extension("toml");

        let toml = TOML::from_path(config_path)?;

        todo!()
    }
}
