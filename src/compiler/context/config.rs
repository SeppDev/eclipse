use std::path::PathBuf;

use crate::diagnostics::{DiagnosticData, DiagnosticResult};

pub const CONFIG_NAME: &str = "eclipse";

#[derive(Default)]
pub struct Config {
    pub package: Package,
    pub editor: Editor,
}

#[derive(Default)]

pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Default)]

pub struct Editor {
    pub tab_size: usize,
}

impl Config {
    pub fn open(path: &PathBuf) -> DiagnosticResult<Self> {
        let mut config_path = path.join(CONFIG_NAME);
        config_path.set_extension("toml");

        if !config_path.exists() {
            return Err(DiagnosticData::basic(
                format!("Failed to find config file on path: {config_path:?}"),
                config_path,
            ));
        }

        // let source = std::fs::read_to_string(config_path).unwrap();
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
