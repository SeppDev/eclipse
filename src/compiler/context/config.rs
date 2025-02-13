use std::path::PathBuf;

use crate::common::errors::CompileResult;

pub const CONFIG_NAME: &str = "eclipse";

pub struct Config {
    pub package: Package,
    pub editor: Editor,
}

pub struct Package {
    pub name: String,
    pub version: String,
}

pub struct Editor {
    pub tab_size: usize,
}

impl Config {
    pub fn open(project_path: &PathBuf) -> CompileResult<Self> {
        let mut config_path = project_path.join(CONFIG_NAME);
        config_path.set_extension("toml");

        let source = std::fs::read_to_string(config_path)?;
        // let json = JSON::from_toml_source(source)?;

        Ok(Config {
            package: Package {
                name: "Application".to_string(),
                version: "0.1.0".to_string(),
            },
            editor: Editor { tab_size: 4 },
        })
    }
}
