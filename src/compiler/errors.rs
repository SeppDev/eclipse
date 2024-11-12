// pub struct CompileError {
//     message: String,
//     lines: Vec<(usize, String)>,
// }
// impl CompileError {

// }

use std::{path::PathBuf, process::exit};

#[derive(Debug, Clone)]
pub struct Location {
    pub lines: Range<usize>,
    pub columns: Range<usize>,
}
impl Location {
    pub fn new(lines: Range<usize>, columns: Range<usize>) -> Self {
        Self { lines, columns }
    }
}

pub struct Messages {
    messages: Vec<Message>,
}
impl Messages {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

pub enum MessageKind {
    Note,
    Warning,
    Error,
}

pub struct Message {
    kind: MessageKind,
    relative_path: PathBuf,
    message: String,
    details: Vec<Detail>,
}
impl Message {
    pub fn new<Message: ToString, Notice: ToString>(
        kind: MessageKind,
        relative_path: &PathBuf,
        message: Message,
        notice: Notice,
        location: &Location
    ) -> Self {
        Self {
            kind,
            relative_path: relative_path.clone(),
            message: message.to_string(),
            details: vec![Detail::new(notice, location)],
        }
    }
    pub fn push<Notice: ToString>(&mut self, notice: Notice, location: &Location) {
        self.details.push(Detail::new(notice, location.clone()));
    }
}

pub struct Detail {
    notice: String,
    location: Location,
}
impl Detail {
    fn new(notice: String, location: Location) -> Self {
        Self { notice, location }
    }
}

fn build_message<T: ToString, E: ToString>(
    message: T,
    notice: E,
    relative_path: &PathBuf,
    location: &Location,
    lines: &Vec<String>,
) -> ! {
    let line = lines.get(location.lines.start - 1).unwrap();

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
        "^".repeat(line.len() - location.columns.start + 1),
        notice.to_string()
    );
    exit(1)
}
