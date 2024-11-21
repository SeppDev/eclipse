mod display;
mod message;

use super::path::Path;
pub use display::*;
pub use message::{Detail, Message, MessageKind};
use std::{collections::HashMap, ops::Range, process::exit};

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
            columns: column..column
        }
    }
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "columns: {}-{}, lines: {}-{}", self.columns.start, self.columns.end, self.lines.start, self.lines.end)
    }
}

type Map = Vec<(Path, Message)>;

#[derive(Default)]
struct Messages {
    notes: Map,
    warnings: Map,
    errors: Map,
}

#[derive(Default)]
pub struct CompileMessages {
    messages: Messages,
    lines: HashMap<Path, Vec<String>>,
}
impl CompileMessages {
    pub fn new() -> Self {
        Self::default()
    }
    fn display(&self, messages: &Map) {
        for (relative_path, msg) in messages {
            let lines = self.lines.get(relative_path).unwrap();
            display_message(relative_path, lines, msg);
        }
    }
    pub fn set_lines(&mut self, relative_path: Path, lines: Vec<String>) {
        self.lines.insert(relative_path, lines);
    }
    pub fn quit(&self) -> ! {
        self.throw(true);
        println!("No debuginfo found, but quitted");
        exit(1)
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
    pub fn push(&mut self, relative_path: Path, message: Message) {
        let vec_to_push = match &message.kind {
            MessageKind::Note => &mut self.messages.notes,
            MessageKind::Warning => &mut self.messages.warnings,
            MessageKind::Error => &mut self.messages.errors,
        };

        vec_to_push.push((relative_path, message));
    }
    pub fn create<T: ToString, E: ToString>(
        &mut self,
        kind: MessageKind,
        location: Location,
        relative_path: Path,
        message: T,
        notice: E,
    ) -> &mut Message {
        let vec_to_push = match &kind {
            MessageKind::Note => &mut self.messages.notes,
            MessageKind::Warning => &mut self.messages.warnings,
            MessageKind::Error => &mut self.messages.errors,
        };

        let message = Message {
            kind,
            message: message.to_string(),
            details: vec![Detail::new(notice.to_string(), location)],
        };
        vec_to_push.push((relative_path, message));
        let (_, message) = vec_to_push.last_mut().unwrap();
        return message;
    }
}

pub fn create_error_message<T: ToString, E: ToString>(
    kind: MessageKind,
    location: Location,
    message: T,
    notice: E,
) -> Message {
    let message = Message {
        kind,
        message: message.to_string(),
        details: vec![Detail::new(notice.to_string(), location)],
    };
    return message;
}
