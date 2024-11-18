use std::time::Instant;

use crate::compiler::errors::Location;

#[derive(Debug)]
pub struct Char {
    pub char: char,
    pub column: usize,
    pub line: usize,
}
impl Char {
    pub fn is_end(&self) -> bool {
        return self.line == 0;
    }
    pub fn to_string(chars: Vec<&Self>) -> Option<String> {
        if chars.len() == 0 {
            return None;
        }

        let mut string = String::new();
        for chr in chars {
            string.push(chr.char);
        }

        return Some(string);
    }
}
impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}, column: {}, line: {}",
            self.char, self.column, self.line
        )
    }
}

#[derive(Debug)]
pub struct Reader {
    pub lines: Vec<String>,
    chars: Vec<Char>,
}
impl Reader {
    pub fn new(source: String) -> Self {
        let mut output = Vec::with_capacity(source.len());
        let mut input: Vec<char> = source.chars().collect();

        let mut lines: Vec<String> = Vec::new();
        let mut line_string: String = String::new();

        let mut line: usize = 1;
        let mut column: usize = 0;

        loop {
            let char = match input.pop() {
                Some(char) => char,
                None => break,
            };

            match char {
                '\r' => continue,
                '\n' => {
                    lines.push(std::mem::take(&mut line_string));
                    line += 1;
                    column = 0;
                }
                '\t' => {
                    column += 4;
                    line_string.push_str("    ");
                }
                ch => {
                    line_string.push(ch);
                    column += 1
                }
            }

            output.push(Char { char, column, line });
        }
        lines.push(line_string);

        // output.reverse();
        Self {
            lines,
            chars: output, //output.into_iter().peekable(),
        }
    }
    fn advance(&mut self) -> Option<Char> {
        self.chars.pop()
    }
    fn peek(&self) -> Option<&Char> {
        self.chars.last()
    }
    pub fn next_string(&mut self) -> Option<(String, TokenKind, Location)> {
        // let mut previous: Option<&Char> = None;

        let start = match self.advance() {
            Some(c) => c,
            None => return None,
        };

        match start.char {
            '"' => {
                let (string, last) = match self.parse_string() {
                    Some(s) => s,
                    None => panic!("Failed to close string"),
                };

                return Some((
                    string,
                    TokenKind::String,
                    Location::new(start.line..last.line, start.column..last.column),
                ));
            }
            '/' => match self.peek() {
                Some(p) => match p.char {
                    '/' => {
                        self.handle_line_comment();
                        return self.next_string();
                    }
                    '*' => todo!(),
                    _ => {}
                },
                None => {}
            },
            '\n' => return self.next_string(),
            _ => {}
        }
        if start.char.is_ascii_alphabetic() {

        } else if start.char.is_ascii_digit() {

        } else if start.char.is_ascii_punctuation() {

        }
        panic!("unkown character: {}", start)
    }
    fn handle_line_comment(&mut self) {
        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => break,
            };
            if char.char == '\n' {
                break;
            }
        }
    }
    fn parse_string(&mut self) -> Option<(String, Char)> {
        let mut body = String::new();
        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => return None,
            };
            match char.char {
                '"' => return Some((body, char)),
                '\\' => {
                    let escape = match self.advance() {
                        Some()
                    };
                },
                chr => body.push(chr)
            }
        };
        
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Comment,
    String,
    Identifier,
    Integer,
    Special,
}
