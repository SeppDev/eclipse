use std::{io::Read, path::PathBuf, process::exit};

use lexer::{Token, TokenInfo};

mod builder;
mod lexer;
mod parser;
mod codegen;
mod analyzer;

pub use builder::build;

pub const FILE_EXTENSION: &str = "eclipse";

pub fn open_file(path: &PathBuf) -> Result<std::fs::File, CompileError> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(CompileError::OpenFile(error)),
    };
    return Ok(file);
}

pub fn read_file(path: &PathBuf) -> Result<String, CompileError> {
    let mut file = match open_file(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(error) => return Err(CompileError::OpenFile(error)),
    }

    return Ok(buf);
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

    // return Ok(String::from_utf8(cmd.stdout).unwrap());
}

// #[derive(Debug)]
// pub enum ParseError {
//     TokensExpectedGot(String),
//     TokensExpected(String),
//     ExpectedExpression,
//     NoTokenFound,
//     Function,
//     Scope,
//     Type,
// }

// #[derive(Debug)]
// pub enum BuildError {
//     AlreadyDefined(String),
//     NotDefined(String),
//     NotMutable(String),
//     WrongMutableType(String),
//     ModuleNotFound,
//     NoNodeFound,
//     TooFewOrManyArguments,
//     WrongReturnType,
//     WrongType,
//     Unkown
// }

#[derive(Debug)]
pub enum BuildError {
    Unkown(String),
    Tokenize(String),
    DuplicateModifier(TokenInfo),
    TokensExpectedGot(Vec<Token>, TokenInfo),
    AlreadyImported(String),
    CannotFindModules([PathBuf; 2]),
    ImportInBlock,
    ExpressionExpected,
    Peekfail,
    NoTokenFound,
}
impl BuildError {
    fn stringify(self) -> String {
        return match self {
            BuildError::TokensExpectedGot(expected, got) => format!(
                "expected: {:?} got: {:?}:{}:{}",
                expected, got.token, got.line, got.column
            ),
            token => format!("{:?}", token),
        };
    }
}

#[derive(Debug)]
pub struct BuildProblem {
    relative_path: PathBuf,
    line: usize,
    error: BuildError,
}
impl BuildProblem {
    pub fn new(error: BuildError, relative_path: PathBuf, line: usize) -> Self {
        Self {
            relative_path,
            line,
            error,
        }
    }
    pub fn print(self) {
        println!("error: {}", self.error.stringify());
        println!(
            "   --> {}:{}",
            self.relative_path.to_string_lossy(),
            self.line
        );
        exit(1)
    }
}

#[derive(Debug)]
pub enum CompileError {
    OpenFile(std::io::Error),
    BuildProblem(BuildProblem),
    GCC(String),
    NASM(String),
}
impl CompileError {
    pub fn print(self) {
        match self {
            CompileError::BuildProblem(problem) => problem.print(),
            CompileError::GCC(msg) => panic!("{}", msg),
            CompileError::NASM(msg) => panic!("{}", msg),
            CompileError::OpenFile(error) => panic!("{:?}", error),
        }
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
