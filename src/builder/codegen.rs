use std::path::PathBuf;

use crate::{parser::parse, FILE_EXTENSION};
use eclipse::CompileError;

pub fn compile(project_path: String) -> Result<String, CompileError> {

    let path = PathBuf::from(&project_path).join(format!("src/main.{}", FILE_EXTENSION));
    let nodes = match parse(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    print!("{:?}", nodes);

    // let file = File

    todo!()
}
