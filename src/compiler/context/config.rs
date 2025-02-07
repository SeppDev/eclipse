use std::path::PathBuf;

use crate::common::{errors::CompileResult, json::JSON};

pub const CONFIG_NAME: &str = "eclipse";
pub const FILE_EXTENSION: &str = "ecl";

pub struct Config {
    pub main_path: PathBuf,
    pub package: Package,
}

pub struct Package {
    pub name: String,
    pub version: String,
}

impl Config {
    pub fn open(project_dir: &PathBuf) -> CompileResult<Self> {
        let mut config_path = project_dir.join(CONFIG_NAME);
        config_path.set_extension("toml");

        let source = std::fs::read_to_string(config_path)?;
        let json = JSON::from_toml_source(source)?;

        let mut main_path = PathBuf::from("./main");
        main_path.set_extension(FILE_EXTENSION);

        Ok(Config {
            main_path,
            package: Package {
                name: "Application".to_string(),
                version: "0.1.0".to_string(),
            },
        })
    }
}
