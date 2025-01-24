use crate::compiler::errors::CompileResult;

use crate::common::location::{Position, PositionRange};

#[derive(Debug, Clone)]
pub struct Char {
    pub char: char,
    pub position: PositionRange,
}

#[derive(Debug)]
pub struct Reader {
    pub lines: Vec<String>,
    chars: Vec<Char>,
}
impl Reader {
    pub fn new(source: String, tab_size: usize) -> Self {
        let mut output = Vec::with_capacity(source.len());
        let mut input: Vec<char> = source.chars().collect();
        input.reverse();

        let mut lines: Vec<String> = Vec::new();
        let mut line_string: String = String::new();

        let mut line: usize = 1;
        let mut column: usize = 0;
        let mut character: usize = 0;

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
                    character = 0;
                }
                '\t' => {
                    column += tab_size;
                    line_string.push_str(" ".repeat(tab_size).as_str());
                }
                ch => {
                    line_string.push(ch);
                    column += 1
                }
            }
            character += 1;

            let position = Position::new(line, column, character);
            let mut range = position.to_range();
            range.end.column += char.len_utf8();
            range.end.character += 1;

            output.push(Char {
                char,
                position: range,
            });
        }
        lines.push(line_string);

        output.reverse();
        Self {
            lines,
            chars: output,
        }
    }
    pub fn advance(&mut self) -> Option<Char> {
        self.chars.pop()
    }
    pub fn peek(&self) -> Option<&Char> {
        self.chars.last()
    }
    pub fn next_string(&mut self) -> CompileResult<Option<TokenKind>> {
        let start = match self.advance() {
            Some(c) => c,
            None => return Ok(None),
        };

        match start.char {
            '"' | '\'' => {
                let (string, last) = match self.parse_string() {
                    Some(a) => a,
                    None => return Err(()),
                };
                return Ok(Some(TokenKind::String(
                    PositionRange::new(start.position.start, last.position.end),
                    string,
                )));
            }
            '/' => match self.peek() {
                Some(p) => match p.char {
                    '/' => {
                        self.handle_line_comment();
                        return self.next_string();
                    }
                    '*' => {
                        self.handle_multi_line_comment();
                        return self.next_string();
                    }
                    _ => {}
                },
                None => {}
            },
            '\n' => return self.next_string(),
            _ => {}
        }

        let mut body = String::from(start.char);
        let mut previous: Char = start.clone();

        if start.char.is_ascii_alphabetic() || start.char == '_' {
            loop {
                let current = match self.peek() {
                    Some(c) => c,
                    None => break,
                };

                if !(current.char.is_ascii_alphabetic()
                    || current.char.is_ascii_digit()
                    || current.char == '_')
                {
                    break;
                }

                let current = self.advance().unwrap();
                body.push(current.char);
                previous = current
            }

            return Ok(Some(TokenKind::Identifier(
                PositionRange::new(start.position.start, previous.position.end),
                body,
            )));
        } else if start.char.is_ascii_punctuation() {
            return Ok(Some(TokenKind::Punctuation(start)));
        } else if start.char.is_ascii_digit() {
            loop {
                let current = match self.peek() {
                    Some(c) => c,
                    None => break,
                };

                if current.char == '.' {
                    self.advance();
                    let float_start = match self.advance() {
                        Some(c) => c,
                        None => break,
                    };
                    if !float_start.char.is_ascii_digit() {
                        break;
                    }
                    let mut decimal = String::from(float_start.char);

                    loop {
                        let current = match self.peek() {
                            Some(c) => c,
                            None => break,
                        };
                        if !current.char.is_ascii_digit() {
                            break;
                        }
                        decimal.push(current.char);
                        previous = self.advance().unwrap();
                    }
                    return Ok(Some(TokenKind::Float(
                        PositionRange::new(start.position.start, previous.position.end),
                        body,
                        decimal,
                    )));
                }

                if !(current.char.is_ascii_digit()) {
                    break;
                }

                let current = self.advance().unwrap();
                body.push(current.char);
                previous = current;
            }

            return Ok(Some(TokenKind::Integer(
                PositionRange::new(start.position.start, previous.position.end),
                body,
            )));
        } else if start.char.is_whitespace() {
            return self.next_string();
        } else {
            panic!("Unkown character: {:?}", start.char)
        }
    }
    fn handle_multi_line_comment(&mut self) {
        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => break,
            };
            if char.char == '*' {
                let char = match self.peek() {
                    Some(c) => c,
                    None => panic!(),
                };
                if char.char != '/' {
                    continue;
                }
                self.advance();
                break;
            }
        }
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
            let current = match self.advance() {
                Some(c) => c,
                None => return None,
            };
            match current.char {
                '"' => return Some((body, current)),
                '\\' => {
                    let escape = match self.advance() {
                        Some(current) => current,
                        None => panic!(),
                    };
                    match escape.char {
                        'n' | 't' | '\\' | '"' | '\'' => todo!(),
                        _ => panic!("Unkown escape character"),
                    }
                }
                chr => body.push(chr),
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    // Comment(String),
    String(PositionRange, String),
    Identifier(PositionRange, String),
    Integer(PositionRange, String),
    Float(PositionRange, String, String),
    Punctuation(Char),
}
