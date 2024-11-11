use std::path::PathBuf;

use analyzer::analyze;
use parser::{parse, ParsedFile};

mod analyzer;
mod lexer;
mod parser;

use program::ParsedProgram;

mod errors;
mod path;
mod program;
mod string;
mod types;

pub static FILE_EXTENSION: &str = "ecl";
pub static POINTER_WIDTH: usize = 8;

fn parse_include(source: &str, name: &str) -> (String, ParsedFile) {
    let mut relative_path = PathBuf::from("std");
    relative_path.push(name);
    relative_path.set_extension(FILE_EXTENSION);

    let mut file = parse(&PathBuf::new(), relative_path, source.to_string());
    file.export = true;
    return (name.to_string(), file);
}

pub fn build(project_dir: PathBuf) {
    let _executable = {
        let std_imports = vec![
            parse_include(include_str!("./std/io.ecl"), "io"),
            parse_include(include_str!("./std/math.ecl"), "math"),
        ];

        let mut standard = ParsedFile::new();
        for (key, file) in std_imports {
            standard.imported.insert(key, file);
        }

        let mut relative_path = PathBuf::from("src/main");
        relative_path.set_extension(FILE_EXTENSION);

        let source = read_file(&project_dir.join(&relative_path));
        let mut main = parse(&project_dir, relative_path, source);
        main.export = true;

        let program = ParsedProgram { standard, main };
        let analyzed = analyze(program);
        println!("{:#?}", analyzed);
    };
}

fn read_file(path: &PathBuf) -> String {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => panic!("{:?}: {:?}", path, error),
    };

    source
}
