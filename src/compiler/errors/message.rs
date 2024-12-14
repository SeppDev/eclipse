use crate::compiler::{path::Path, FILE_EXTENSION};

use super::{CompileCtx, Location, Map};

#[derive(Debug, PartialEq)]
pub enum MessageVariant {
    Note,
    Warning,
    Error,
}
impl std::fmt::Display for MessageVariant {
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
    pub variant: MessageVariant,
    message: String,
    details: Vec<Detail>,
}
impl Message {
    pub fn error(message: String) -> Self {
        Self {
            variant: MessageVariant::Error,
            message,
            details: Vec::new(),
        }
    }
    pub fn warning(message: String) -> Self {
        Self {
            variant: MessageVariant::Warning,
            message,
            details: Vec::new(),
        }
    }
    pub fn note(message: String) -> Self {
        Self {
            variant: MessageVariant::Note,
            message,
            details: Vec::new(),
        }
    }
    pub fn set_notice<Notice: ToString>(&mut self, notice: Notice) {
        let detail = self.details.first_mut().unwrap();
        detail.set_notice(notice);
    }
    pub fn push<Notice: ToString>(&mut self, notice: Notice, location: Location) -> &mut Detail {
        self.details.push(Detail::new(notice.to_string(), location));
        self.details.last_mut().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Detail {
    pub notice: String,
    pub location: Location,
}
impl Detail {
    pub fn set_notice<Notice: ToString>(&mut self, notice: Notice) {
        self.notice = notice.to_string();
    }
    pub fn new(notice: String, location: Location) -> Self {
        Self { notice, location }
    }
}

impl CompileCtx {
    pub fn display(&self, messages: &Map) {
        for (relative_path, msg) in messages {
            let lines = self.lines.get(relative_path).unwrap();
            display(relative_path, msg, lines);
        }
    }
}

fn display(relative_path: &Path, message: &Message, lines: &Vec<String>) {
    println!("{}: {}", message.variant, message.message);

    let first = message.details.first().unwrap();
    println!(
        "  --> {}.{}:{}:{}",
        relative_path.convert().to_string_lossy(),
        FILE_EXTENSION,
        first.location.lines.start,
        first.location.columns.start
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

        let repeat: usize = {
            if location.lines.len() > 1 {
                line.len() - location.columns.start + 1
            } else {
                (location.columns.end - location.columns.start).max(1)
            }
        };

        println!(" {} |", spacing);
        println!(" {}{} | {}", line_spacing, location.lines.start, line);
        println!(
            " {} | {}{} {}",
            spacing,
            " ".repeat(location.columns.start.max(1) - 1),
            "^".repeat(repeat),
            detail.notice
        );
    }
    println!()
}
