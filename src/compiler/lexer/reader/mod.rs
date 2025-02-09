pub mod comments;
pub mod identifier;
pub mod next;
pub mod number;
pub mod operators;
pub mod string;

use crate::{
    common::{errors::CompileResult, located::Located, position::Position},
    compiler::context::CompileCtx,
};

pub struct Reader {
    chars: Vec<Character>,
}

pub type Character = Located<char>;

impl CompileCtx {
    pub(in super::super) fn new_reader(&self, source: &String) -> CompileResult<Reader> {
        let tab_size = self.config.editor.tab_size;

        let mut input = source.chars();
        let mut output: Vec<Character> = Vec::with_capacity(source.len());

        let mut line: usize = 1;
        let mut column: usize = 0;
        let mut character: usize = 0;

        loop {
            let char = match input.next() {
                Some(char) => char,
                None => break,
            };

            match char {
                '\r' => continue,
                '\n' => {
                    // lines.push(std::mem::take(&mut line_string));
                    line += 1;
                    column = 0;
                    character = 0;
                }
                '\t' => {
                    column += tab_size;
                    // line_string.push_str(" ".repeat(tab_size).as_str());
                }
                _ => {
                    // line_string.push(ch);
                    column += 1
                }
            }
            character += 1;

            let single_position = Position::new(line, column, character);
            let mut position = single_position.to_range();
            position.end.column += char.len_utf8();
            position.end.character += 1;

            output.push(Character::new(char, position));
        }
        output.reverse();

        Ok(Reader { chars: output })
    }
}
impl Reader {
    pub fn advance(&mut self) -> Option<Character> {
        self.chars.pop()
    }
    pub fn advance_if<F>(&mut self, predicate: F) -> Option<Character>
    where
        F: FnOnce(&char) -> bool,
    {
        let peeked = match self.peek() {
            Some(p) => p,
            None => return None,
        };

        if predicate(&peeked.raw) {
            return self.advance();
        }
        None
    }
    pub fn peek(&self) -> Option<&Character> {
        self.chars.last()
    }
}
