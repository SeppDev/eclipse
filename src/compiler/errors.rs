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
    files: Vec<FileMessages>,
}
impl CompileMessages {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }
    pub fn create(&mut self) -> FileMessages {
        let file = FileMessages::new();
        return file;
    }
    pub fn push(&mut self, file: FileMessages) {
        self.files.push(file)
    }
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

#[derive(Debug, Default)]
pub struct FileMessages {
    messages: Vec<Message>,
    relative_path: Path,
    lines: Vec<String>,
}
impl FileMessages {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_path(&mut self, path: Path) {
        self.relative_path = path
    }
    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.lines = lines
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

fn display_message(relative_path: &Path, lines: &Vec<String>, message: &Message) {
    println!("{}: {}", message.kind, message.message);

    let first = message.details.first().unwrap();
    println!(
        "  --> {}:{}:{}",
        relative_path, first.location.lines.start, first.location.columns.start
    );

    let mut spacing = String::new();
    for detail in &message.details {
        let location = &detail.location;
        let temp_spacing = String::from(" ").repeat(format!("{}", location.lines.start).len());

        if temp_spacing.len() > spacing.len() {
            spacing = temp_spacing;
        }
    }

    for detail in &message.details {
        let location = &detail.location;
        let line = lines.get(location.lines.start - 1).unwrap();
        let total_spacing = format!("{}", detail.location.lines.start).len();
        let line_spacing = String::from(" ").repeat(spacing.len() - total_spacing);

        println!(" {} |", spacing);
        println!(" {}{} | {}", line_spacing, location.lines.start, line);
        println!(
            " {} | {}{} {}",
            spacing,
            " ".repeat(location.columns.start - 1),
            "^".repeat(line.len() - location.columns.start + 1),
            detail.notice
        );
    }
    println!()

    // let line = lines.get(location.lines.start - 1).unwrap();

    // println!("error: {}", message.to_string());

    // println!("  | {}", line);
}
