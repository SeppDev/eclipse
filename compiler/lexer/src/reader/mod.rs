use common::position::{LocatedAt, Position};

pub mod comments;
pub mod identifier;
pub mod next;
pub mod number;
pub mod operators;
pub mod string;

pub const TAB_SIZE: usize = 4;

pub struct Reader {
    chars: Vec<Character>,
}

pub type Character = LocatedAt<char>;

impl Reader {
    pub fn new(source: &str) -> Self {
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
                    line += 1;
                    column = 0;
                    character = 0;
                }
                '\t' => {
                    column += TAB_SIZE;
                }
                _ => column += 1,
            }
            character += 1;

            let single_position = Position::new(line, column, character);
            let mut position = single_position.to_range();
            position.end.column += char.len_utf8();
            position.end.character += 1;

            output.push(Character::new(char, position));
        }
        output.reverse();

        Self { chars: output }
    }

    pub fn advance(&mut self) -> Option<Character> {
        self.chars.pop()
    }
    pub fn advance_if<P>(&mut self, predicate: P) -> Option<Character>
    where
        P: FnOnce(&char) -> bool,
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
    pub fn peek_second(&self) -> Option<&Character> {
        self.chars.get(self.chars.len() - 2)
    }
}
