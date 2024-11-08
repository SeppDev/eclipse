use std::path::PathBuf;

use parser::parse;

mod analyzer;
mod lexer;
mod parser;

mod benchmark;
pub use benchmark::*;
use program::ParsedProgram;

mod errors;
mod path;
mod program;
mod string;
mod types;

pub static FILE_EXTENSION: &str = "ecl";

pub fn build(project_dir: PathBuf) {
    let current_dir = std::env::current_exe().unwrap();
    let current_dir = current_dir.parent().unwrap().to_path_buf();
    println!("{:#?}", current_dir);

    let _executable = {
        let mut relative_path = PathBuf::from("std/lib");
        relative_path.set_extension(FILE_EXTENSION);
        let standard = parse(&current_dir, relative_path);

        let mut relative_path = PathBuf::from("src/main");
        relative_path.set_extension(FILE_EXTENSION);
        
        let main = parse(&project_dir, relative_path);
        let program = ParsedProgram {
            standard,
            main,
        };
        println!("{:#?}", program);
    };
}

fn read_file(path: &PathBuf) -> String {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => panic!("{:?}: {:?}", path, error),
    };

    source
}
