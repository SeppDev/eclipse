use std::path::PathBuf;

use crate::common::{errors::CompileResult, json::JSON};

pub const CONFIG_NAME: &str = "eclipse";

pub struct Config {
    pub name: String,
    pub version: String,
}

impl Config {
    pub fn open(project_dir: &PathBuf) -> CompileResult<Self> {
        let mut config_path = project_dir.join(CONFIG_NAME);
        config_path.set_extension("toml");

        let source = std::fs::read_to_string(config_path)?;
        let json = JSON::from_toml_source(source)?;

        println!("{json:#?}");

        todo!()
    }
}
