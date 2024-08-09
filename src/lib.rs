use std::io::Read;

pub fn open_file(path: &str) -> Result<std::fs::File, BuildError> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(BuildError::OpenFile(error)),
    };
    return Ok(file);
}

pub fn read_file(path: &str) -> Result<String, BuildError> {
    let mut file = match open_file(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(error) => return Err(BuildError::OpenFile(error)),
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
pub enum BuildError {
    OpenFile(std::io::Error),
    Parsing,
    Building,
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
