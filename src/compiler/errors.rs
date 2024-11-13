// pub struct CompileError {
//     message: String,
//     lines: Vec<(usize, String)>,
// }
// impl CompileError {

// }

use std::{ops::Range, process::exit};

use super::path::Path;

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
    pub fn push(&mut self, messages: Self) {
        for msg in messages.messages {
            self.messages.push(msg);
        }
    }
    pub fn get_messages(self) -> Vec<Message> {
        return self.messages;
    }
    pub fn create<T: ToString, E: ToString>(
        &mut self,
        kind: MessageKind,
        relative_path: Path,
        location: Location,
        message: T,
        notice: E,
    ) -> &mut Message {
        Message {
            kind,
            relative_path,
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
    relative_path: Path,
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
    relative_path: &Path,
    location: &Location,
    lines: &Vec<String>,
) -> ! {
    let line = lines.get(location.lines.start - 1).unwrap();

    println!("error: {}", message.to_string());
    println!(
        "  --> {}:{}:{}",
        relative_path,
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
