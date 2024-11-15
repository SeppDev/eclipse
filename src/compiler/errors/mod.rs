mod display;
mod message;

pub use display::*;
use message::{Detail, Message, MessageKind};
use super::path::Path;
use std::{collections::{BTreeMap, HashMap}, ops::Range, process::exit};

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
    files: BTreeMap<Path, FileMessages>,
    lines: HashMap<Path, Vec<String>>
}
impl CompileMessages {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_file(&mut self, relative_path: Path, lines: Vec<String>) -> &mut FileMessages {
        self.files.insert(relative_path, FileMessages::new());
        self.files.last_mut().unwrap()
    }
    fn has_errors(&self) -> bool {
        for file in &self.files {
            if file.has_errors() {
                return true;
            }
        }
        return false;
    }
    pub fn throw(&self, finish: bool) {
        let has_errors = self.has_errors();
        if !has_errors || finish {
            return;
        }
        for file in &self.files {
            file.throw();
        }
        if has_errors {
            exit(1)
        }
    }
}


#[derive(Debug)]
pub struct FileMessages {
    messages: Vec<Message>,
}
impl FileMessages {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
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
            display_message(&self.relative_path, message)
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

