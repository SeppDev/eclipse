use std::{collections::BTreeMap, path::PathBuf};

use crate::common::path::Path;

use super::{
    errors::CompileResult,
    json::{self, JSON},
};

pub struct TOML {
    sections: BTreeMap<Key, Table>,
}
impl TOML {
    pub fn from_path(path: PathBuf) -> CompileResult<Self> {
        let source = std::fs::read_to_string(path)?;
        return Self::from(source);
    }
    pub fn from(source: String) -> CompileResult<Self> {
        let mut split = source.split("\n");
        let mut current_table: JSON = JSON::new();

        loop {
            let line = match split.next() {
                Some(l) => l,
                None => break,
            };
            if line.len() == 0 {
                continue;
            }

            let mut converted;
            let is_array = if line.starts_with("[[") {
                converted = line.replace("]]", "");
                converted = converted.replace("[[", "");
                true
            } else if line.starts_with("[") {
                converted = line.replace("[", "");
                converted = converted.replace("]", "");
                false
            } else {
                todo!()
            };

            let path = converted.split(".").collect::<Vec<&str>>();
            if is_array {
                todo!();
                continue;
            }
        }

        todo!()
    }
}

fn parse_dot_path() -> Path {
    todo!()
}

#[derive(Hash)]
pub enum Key {
    Literal(String),
}
pub struct Section {}

pub struct Table {
    values: BTreeMap<Key, JSON>,
}
