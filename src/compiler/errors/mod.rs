mod message;

use super::path::Path;
pub use message::{Message, MessageKind};
use std::{borrow::BorrowMut, collections::HashMap, ops::Range, process::exit};

pub type CompileResult<T> = Result<T, ()>;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub lines: Range<usize>,
    pub columns: Range<usize>,
}
impl Location {
    pub fn new(lines: Range<usize>, columns: Range<usize>) -> Self {
        Self { lines, columns }
    }
    pub fn single(line: usize, column: usize) -> Self {
        Self {
            lines: line..line,
            columns: column..column,
        }
    }
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "columns: {}-{}, lines: {}-{}",
            self.columns.start, self.columns.end, self.lines.start, self.lines.end
        )
    }
}

type Map = Vec<(Path, Message)>;

#[derive(Debug, Default)]
struct MsgMap {
    notes: Map,
    warnings: Map,
    errors: Map,
}

#[derive(Debug, Default)]
pub struct CompileCtx {
    messages: MsgMap,
    lines: HashMap<Path, Vec<String>>,
    current_file_path: Path,
}
impl CompileCtx {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_lines(&mut self, relative_path: Path, lines: Vec<String>) {
        self.lines.insert(relative_path, lines);
    }
    pub fn quit(&self) -> ! {
        self.throw(true);
        println!("No debuginfo found, but quitted");
        exit(1)
    }
    pub fn push(&mut self, relative_file_path: Path, message: Message) {
        match &message.kind {
            MessageKind::Note => self.messages.errors.push((relative_file_path, message)),
            MessageKind::Warning => self.messages.warnings.push((relative_file_path, message)),
            MessageKind::Error => self.messages.errors.push((relative_file_path, message)),
        }
    }
    pub fn throw(&self, finish: bool) {
        let has_errors = self.messages.errors.len() > 0;
        if !has_errors && !finish {
            return;
        }

        self.display(&self.messages.notes);
        self.display(&self.messages.warnings);
        self.display(&self.messages.errors);

        if has_errors {
            exit(1)
        }
    }

    pub fn error<T: ToString>(
        &mut self,
        location: Location,
        message: T,
    ) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.messages.errors.push((self.current_file_path.clone(), message));
        return self.messages.errors.last_mut().unwrap().1.borrow_mut();
    }
    pub fn warning<T: ToString>(
        &mut self,
        location: Location,
        message: T,
    ) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.messages.errors.push((self.current_file_path.clone(), message));
        return self.messages.warnings.last_mut().unwrap().1.borrow_mut();
    }
    pub fn note<T: ToString>(
        &mut self,
        location: Location,
        message: T,
    ) -> &mut Message {
        let mut message = Message::error(message.to_string());
        message.push("", location);
        self.messages.errors.push((self.current_file_path.clone(), message));
        return self.messages.notes.last_mut().unwrap().1.borrow_mut();
    }
}
