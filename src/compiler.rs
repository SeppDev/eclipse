use std::path::PathBuf;

use parser::{parse, ParsedFile};

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

fn parse_include(source: &str, name: &str) -> ParsedFile {
    let mut relative_path = PathBuf::from("std");
    relative_path.push(name);
    relative_path.set_extension(FILE_EXTENSION);

    println!("{:?}", relative_path);

    let file = parse(&PathBuf::new(), relative_path, source.to_string());
    return file;
}

pub fn build(project_dir: PathBuf) {
    let _executable = {
        let std_imports = vec![
            parse_include(include_str!("./std/io.ecl"), "io"),
            parse_include(include_str!("./std/math.ecl"), "math"),
        ];

        let mut standard = ParsedFile::new();
        

        let mut relative_path = PathBuf::from("src/main");
        relative_path.set_extension(FILE_EXTENSION);

        let source = read_file(&project_dir.join(&relative_path));
        let main = parse(&project_dir, relative_path, source);
        let program = ParsedProgram { standard, main };
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
