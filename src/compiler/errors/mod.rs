mod display;
pub use display::*;

use super::path::Path;
use std::{ops::Range, process::exit};

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
    files: Vec<FileMessages>,
}
impl CompileMessages {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }
    pub fn push_file(&mut self, file: FileMessages) {
        self.files.push(file);
    }
    // pub fn create_file(&mut self, relative_path: Path, lines: Vec<String>) -> &mut FileMessages {
    //     self.files.push(FileMessages::new(relative_path, lines));
    //     self.files.last_mut().unwrap()
    // }

    fn has_errors(&self) -> bool {
        for file in &self.files {
            if file.has_errors() {
                return true;
            }
        }
        return false;
    }
    pub fn should_throw(&self) {
        let has_errors = self.has_errors();
        if !has_errors {
            return;
        }
        for file in &self.files {
            file.throw();
        }
        if has_errors {
            exit(1)
        }
    }
    pub fn throw(&self) {
        for file in &self.files {
            file.throw();
        }
        if self.has_errors() {
            exit(1)
        }
    }
}

#[derive(Debug)]
pub struct FileMessages {
    messages: Vec<Message>,
    relative_path: Path,
    lines: Vec<String>,
}
impl FileMessages {
    pub fn new(relative_path: Path, lines: Vec<String>) -> Self {
        Self {
            messages: Vec::new(),
            lines,
            relative_path,
        }
    }
    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.lines = lines;
    }
    fn has_errors(&self) -> bool {
        for message in &self.messages {
            if message.kind.eq(&MessageKind::Error) {
                return true;
            }
        }
        return false;
    }
    fn throw(&self) {
        for message in &self.messages {
            display_message(&self.relative_path, &self.lines, message)
        }
    }
    // pub fn push(&mut self, message: Message) {
    //     self.messages.push(message)
    // }
    pub fn create<T: ToString, E: ToString>(
        &mut self,
        kind: MessageKind,
        location: Location,
        message: T,
        notice: E,
    ) -> &mut Message {
        let message = Message {
            kind,
            message: message.to_string(),
            details: vec![Detail::new(notice.to_string(), location)],
        };
        self.messages.push(message);
        self.messages.last_mut().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Detail {
    notice: String,
    location: Location,
}
impl Detail {
    fn new(notice: String, location: Location) -> Self {
        Self { notice, location }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageKind {
    Note,
    Warning,
    Error,
}
impl std::fmt::Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    kind: MessageKind,
    message: String,
    details: Vec<Detail>,
}
impl Message {
    pub fn push<Notice: ToString>(&mut self, notice: Notice, location: Location) {
        self.details.push(Detail::new(notice.to_string(), location));
    }
}
