use crate::{
    common::location::PositionRange,
    compiler::{path::Path, FILE_EXTENSION},
};

use super::{CompileCtx, Map};

#[derive(Debug, PartialEq)]
pub enum MessageVariant {
    Warning,
    Error,
}
impl std::fmt::Display for MessageVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}
#[derive(Debug)]
pub struct CompileMessage {
    pub variant: MessageVariant,
    message: String,
    details: Vec<Detail>,
}
impl CompileMessage {
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
    pub fn set_notice<Notice: ToString>(&mut self, notice: Notice) {
        let detail = self.details.first_mut().unwrap();
        detail.set_notice(notice);
    }
    pub fn push<Notice: ToString>(
        &mut self,
        notice: Notice,
        position: PositionRange,
    ) -> &mut Detail {
        self.details.push(Detail::new(notice.to_string(), position));
        self.details.last_mut().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Detail {
    pub notice: String,
    pub position: PositionRange,
}
impl Detail {
    pub fn set_notice<Notice: ToString>(&mut self, notice: Notice) {
        self.notice = notice.to_string();
    }
    pub fn new(notice: String, position: PositionRange) -> Self {
        Self { notice, position }
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

fn display(relative_path: &Path, message: &CompileMessage, lines: &Vec<String>) {
    println!("{}: {}", message.variant, message.message);

    let first = message.details.first().unwrap();
    println!(
        "  --> {}.{}:{}:{}",
        relative_path.into_path_buf().to_string_lossy(),
        FILE_EXTENSION,
        first.position.start.line,
        first.position.start.column
    );

    let mut spacing = String::new();
    for detail in &message.details {
        let location = &detail.position;
        let temp_spacing = String::from(" ").repeat(format!("{}", location.start.line).len());

        if temp_spacing.len() > spacing.len() {
            spacing = temp_spacing;
        }
    }

    for detail in &message.details {
        let position = &detail.position;
        let line = lines.get(position.start.line - 1).unwrap();
        let total_spacing = format!("{}", detail.position.start.line).len();
        let line_spacing = String::from(" ").repeat(spacing.len() - total_spacing);

        let repeat: usize = {
            // if location.lines.len() > 1 {
            // line.len() - location.columns.start + 1
            // } else {
            (position.end.column - position.start.column).max(1)
            // }
        };

        println!(" {} |", spacing);
        println!(" {}{} | {}", line_spacing, position.start.line, line);
        println!(
            " {} | {}{} {}",
            spacing,
            " ".repeat(position.start.column.max(1) - 1),
            "^".repeat(repeat),
            detail.notice
        );
    }
    println!()
}
