use std::io::Read;

pub fn open_file(path: &str) -> Result<std::fs::File, CompileError> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(CompileError::OpenFile(error)),
    };
    return Ok(file);
}

pub fn read_file(path: &str) -> Result<String, CompileError> {
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

    return Ok(String::from_utf8(cmd.stdout).unwrap());
}

#[derive(Debug)]
pub enum ParseError {
    TokenExpected(String),
    NoTokenFound,
    Function,
    Scope,
    Type,
}

#[derive(Debug)]
pub enum BuildError {
    TooFewOrManyArguments,
    AlreadyDefined(String),
    NotDefined(String),
    NotMutable(String),
    WrongMutableType(String),
    WrongReturnType,
    WrongType,
    Unkown
}

#[derive(Debug)]
pub enum CompileError {
    OpenFile(std::io::Error),
    
    Building(BuildError),
    Parsing(ParseError),

    GCC(String),
    NASM(String)
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
