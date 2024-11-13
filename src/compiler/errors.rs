// pub struct CompileError {
//     message: String,
//     lines: Vec<(usize, String)>,
// }
// impl CompileError {

// }

use std::{ops::Range, path::PathBuf, process::exit};

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

#[derive(Debug, Default)]
pub struct CompileMessages {
    messages: Vec<Message>,
}
impl CompileMessages {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}
impl CompileMessages {
    pub fn create<T: ToString, E: ToString>(
        &mut self,
        kind: MessageKind,
        relative_path: PathBuf,
        message: T,
        notice: E,
        location: Location,
    ) -> &mut Message {
        Message {
            kind,
            relative_path: relative_path.clone(),
            message: message.to_string(),
            details: vec![Detail::new(notice.to_string(), location)],
        };
        self.messages.last_mut().unwrap()
    }
}

#[derive(Debug)]
pub enum MessageKind {
    Note,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct Message {
    kind: MessageKind,
    relative_path: PathBuf,
    message: String,
    details: Vec<Detail>,
}
impl Message {
    pub fn push<Notice: ToString>(&mut self, notice: Notice, location: Location) {
        self.details.push(Detail::new(notice.to_string(), location));
    }
}

#[derive(Debug)]
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
