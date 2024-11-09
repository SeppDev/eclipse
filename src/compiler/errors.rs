// pub struct CompileError {
//     message: String,
//     lines: Vec<(usize, String)>,
// }
// impl CompileError {

// }

use std::{path::PathBuf, process::exit};

use super::lexer::Location;

pub fn throw_error<T: ToString>(
    message: T,
    relative_path: &PathBuf,
    location: &Location,
    lines: &Vec<String>,
) -> ! {
    let line = match lines.get(location.lines.start - 1) {
        Some(s) => s,
        None => panic!("Could not find: {:?}", location),
    };

    println!("error: {}", message.to_string());
    println!(
        "  --> {}:{}:{}",
        relative_path.to_string_lossy(),
        location.lines.start,
        location.columns.start
    );

    println!("  |");
    println!("  | {}", line);
    println!(
        "  | {}{} {}",
        " ".repeat(location.columns.start - 1),
        "^".repeat(location.columns.end - location.columns.start),
        ""
    );
    exit(1)
}
