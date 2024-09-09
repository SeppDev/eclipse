use crate::parser::lexer::token::{Token, TokenInfo};

pub struct Reader {
    pub tokens: Vec<TokenInfo>,
    chars: Vec<char>,
    index: usize,

    column: usize,
    line: usize,
}
impl Reader {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            tokens: Vec::new(),
            chars,
            index: 0,
            column: 1,
            line: 1
        }
    }
    pub fn next(&mut self) -> Option<String> {
        let mut string = String::new();
        loop {
            match self.chars.get(self.index) {
                Some(schar) => {
                    self.index += 1;
                    self.column += 1;

                    string.push(schar.clone());
                    match schar {
                        '\n' => {
                            self.line += 1;
                            self.column = 1;
                            break
                        },
                        '\r' => break,
                        '\t' => break,
                        ' ' => break,
                        _ => continue,
                    }
                }
                None => break,
            }
        }
        if string.len() == 0 {
            return None;
        }
        return Some(string);
    }
    pub fn push(&mut self, token: Token) {
        self.tokens.push(TokenInfo::new(token, self.line, self.column))
    }
}
