use crate::compiler::path::Path;

use super::Message;

pub fn display_message(relative_path: &Path, lines: &Vec<String>, message: &Message) {
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
            "^".repeat(location.columns.end - location.columns.start + 1),
            detail.notice
        );
    }
    println!()
}
