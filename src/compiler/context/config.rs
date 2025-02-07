use std::path::PathBuf;

use crate::common::{errors::CompileResult, files::FILE_EXTENSION, json::JSON};

pub const CONFIG_NAME: &str = "eclipse";

pub struct Config {
    pub project_path: PathBuf,
    pub package: Package,
}

pub struct Package {
    pub name: String,
    pub version: String,
}

impl Config {
    pub fn open(project_path: PathBuf) -> CompileResult<Self> {
        let mut config_path = project_path.join(CONFIG_NAME);
        config_path.set_extension("toml");

        let source = std::fs::read_to_string(config_path)?;
        let json = JSON::from_toml_source(source)?;

        Ok(Config {
            project_path,
            package: Package {
                name: "Application".to_string(),
                version: "0.1.0".to_string(),
            },
        })
    }
}
