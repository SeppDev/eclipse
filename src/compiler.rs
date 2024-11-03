use std::path::PathBuf;

use parser::parse;

mod lexer;
mod parser;

mod errors;
mod path;
mod string;
mod types;

pub static FILE_EXTENSION: &str = "ecl";

pub fn build(project_dir: PathBuf) {
    let _main = {
        let mut relative_path = PathBuf::from("src/main");
        relative_path.set_extension(FILE_EXTENSION);

        parse(&project_dir, relative_path);
    };

}

fn read_file(path: &PathBuf) -> String {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => todo!("{:?}", error),
    };

    source
}
