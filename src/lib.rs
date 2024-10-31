use std::{io::Read, path::PathBuf, process::{exit, Output}};

use analyzer::{analyze, RandomString};
use codegen::builder;
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

pub fn execute(command: String) -> Result<Output, String> {
    let cmd = match std::process::Command::new("cmd")
        .args(["/C", &command])
        .output()
    {
        Ok(a) => a,
        Err(a) => return Err(a.to_string()),
    };

    return Ok(cmd)
}

pub fn build(project_path: PathBuf) -> Result<PathBuf, CompileError> {
    let mut random_string = RandomString::new();

    let main = ASTModule::new(&project_path, &PathBuf::from("src/main"))?;
    let program = analyze(main, &mut random_string)?;
    let executable_path = builder::codegen(&project_path, program, builder::Mode::LLVM, random_string);

    return Ok(executable_path);
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
