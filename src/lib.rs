use std::{io::Read, path::PathBuf, process::exit};

use analyzer::analyze;
use codegen::generate;
use parser::*;

mod lexer;
mod parser;
mod analyzer;
mod codegen;

pub static FILE_EXTENSION: &str = "ecl";

pub fn open_file(path: &PathBuf) -> std::fs::File {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("{:?}", error),
    };
    return file;
}

pub fn read_file(path: &PathBuf) -> String {
    let mut file = open_file(path);

    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(error) => panic!("{:?}", error),
    }

    return buf;
}

pub fn execute(command: String) -> Result<String, String> {
    let cmd = match std::process::Command::new("cmd")
        .args(["/C", &command])
        .output()
    {
        Ok(a) => a,
        Err(a) => return Err(a.to_string()),
    };

    if cmd.stderr.len() > 0 {
        let mut result = command.clone();
        result.push_str("\n");
        result.push_str(String::from_utf8(cmd.stderr).unwrap().as_str());
        return Err(result);
    }

    return Ok(String::from_utf8(cmd.stdout).unwrap());
}

pub fn build(project_path: PathBuf) -> Result<PathBuf, CompileError> {
    let main = ASTModule::new(&project_path, &PathBuf::from("src/main"))?;
    analyze(main)?;
    // generate(module, types);

    // println!("{:#?}", main);

    todo!()
}

pub type AnalyzeResult<T> = Result<T, CompileError>;
pub type ParseResult<T> = Result<T, CompileError>;

// let relative_path = PathBuf::from(relative_path.to_string_lossy().replace("\\", "/"));

#[derive(Debug)]
pub struct CompileError {
    error: String,
    line: usize
}
impl CompileError {
    pub fn new(error: String, line: usize) -> Self {
        Self {
            line,
            error
        }
    }
    pub fn print(&self) {
        println!("error: {}", self.error);
        println!("line: {:?}", self.line);

        exit(1)
    }
}

pub enum BuildError {
    OpenFile(std::io::Error),
    CompileError(CompileError),
    // ParseError(ParseError),
    GCC(String),
    NASM(String),
}
impl BuildError { 
    pub fn print(self) {
        match self {
            BuildError::CompileError(problem) => problem.print(),
            BuildError::GCC(msg) => panic!("{}", msg),
            BuildError::NASM(msg) => panic!("{}", msg),
            BuildError::OpenFile(error) => panic!("{:?}", error),
        }
        exit(1);
    }
}

// macro_rules! warn {
//     () => {
//         $crate::print!("\n")
//     };
//     ($($arg:tt)*) => {{
//         $crate::io::_print($crate::format_args_nl!($($arg)*));
//     }};
// }

// macro_rules! log {
//     ($($arg:tt)*) => {{

//         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
//         res
//     }}
// }
