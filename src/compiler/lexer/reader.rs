use core::panic;

use crate::compiler::errors::{CompileResult, Location};

#[derive(Debug, Clone)]
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

pub fn read_source(source: String) -> CompileResult<Vec<(Location, TokenKind)>> {
    let mut reader = Reader::new(source);
    let mut tokens = Vec::new();
    loop {
        let (location, token) = match reader.next_string()? {
            Some((l, t)) => (l, t),
            None => break,
        };
        tokens.push((location, token));
    }
    return Ok(tokens);
}

#[derive(Debug)]
struct Reader {
    pub lines: Vec<String>,
    chars: Vec<Char>,
}
impl Reader {
    pub fn new(source: String) -> Self {
        let mut output = Vec::with_capacity(source.len());
        let mut input: Vec<char> = source.chars().collect();
        input.reverse();

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

        output.reverse();
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
    pub fn next_string(&mut self) -> CompileResult<Option<(Location, TokenKind)>> {
        // let mut previous: Option<&Char> = None;

        let start = match self.advance() {
            Some(c) => c,
            None => return Ok(None),
        };

        match start.char {
            '"' | '\'' => {
                let (string, last) = match self.parse_string() {
                    Ok(s) => s,
                    Err(()) => panic!("Failed to close string"),
                };

                return Ok(Some((
                    Location::new(start.line..last.line, start.column..last.column),
                    TokenKind::String(string),
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
                    None => return Err(()),
                };
                if !(current.char.is_ascii_alphabetic()
                    || current.char.is_ascii_digit()
                    || current.char == '_')
                {
                    break;
                }

                let current = self.advance().unwrap();
                body.push(current.char);
                previous = current;
            }
            return Ok(Some((
                Location::new(start.line..previous.line, start.column..previous.column),
                TokenKind::Identifier(body),
            )));
        } else if start.char.is_ascii_digit() {
            loop {
                let current = match self.peek() {
                    Some(c) => c,
                    None => return Err(()),
                };
                
                if current.char == '.' {
                    self.advance();
                    let char = match self.advance() {
                        Some(c) => c,
                        None => return Err(()),
                    };
                    if !char.char.is_ascii_digit() {
                        return Err(());
                    }

                    let mut decimal = String::from(char.char);
                    loop {
                        let current = match self.peek() {
                            Some(c) => c,
                            None => panic!(),
                        };
                        if !(current.char.is_ascii_digit()) {
                            if current.char.is_whitespace() {
                                break;
                            }
                            return Err(());
                        }
                        let current = self.advance().unwrap();
                        decimal.push(current.char);
                        previous = current;
                    }
                    return Ok(Some((
                        Location::new(start.line..previous.line, start.column..previous.column),
                        TokenKind::Float(body, decimal),
                    )));
                }
                if !(current.char.is_ascii_digit()) {

                    if current.char.is_ascii_whitespace() {
                        break;
                    }
                    return Err(());
                }

                let current = self.advance().unwrap();
                body.push(current.char);
                previous = current;
            }
            return Ok(Some((
                Location::new(start.line..previous.line, start.column..previous.column),
                TokenKind::Integer(body),
            )));
        } else if start.char.is_ascii_punctuation() {
            return Ok(Some((
                Location::new(start.line..previous.line, start.column..previous.column),
                TokenKind::Punctuation(start.char),
            )));
        } else if start.char.is_whitespace() {
            return self.next_string();
        } else {
            panic!("Unkown character: {}", start)
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
    fn parse_string(&mut self) -> CompileResult<(String, Char)> {
        let mut body = String::new();
        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => return Err(()),
            };
            match char.char {
                '"' => return Ok((body, char)),
                '\\' => {
                    let escape = match self.advance() {
                        Some(char) => char,
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
    String(String),
    Identifier(String),
    Integer(String),
    Float(String, String),
    Punctuation(char),
}
