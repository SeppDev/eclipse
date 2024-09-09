use std::path::PathBuf;

use crate::analyzer::analyze;
use crate::parser::Program;
use crate::CompileError;

use crate::FILE_EXTENSION;

pub fn build(project_path: PathBuf) -> Result<String, CompileError> {
    let mut program = Program::new(project_path);
    match program.parse(PathBuf::from(format!("src/main.{}", FILE_EXTENSION))) {
        Ok(()) => {}
        Err(error) => return Err(error),
    };

    analyze(program).unwrap();


    todo!()
}
