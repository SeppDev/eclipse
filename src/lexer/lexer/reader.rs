use crate::lexer::token::{Token, TokenInfo};

pub struct Tokens {
    pub tokens: Vec<TokenInfo>,
    chars: Vec<char>,
    index: usize,
    line: usize,
}
impl Tokens {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            tokens: Vec::new(),
            chars,
            index: 0,
            line: 1
        }
    }
    pub fn next(&mut self) -> Option<String> {
        let mut string = String::new();
        loop {
            match self.chars.get(self.index) {
                Some(schar) => {
                    self.index += 1;

                    string.push(schar.clone());
                    match schar {
                        '\n' => {
                            self.line += 1;
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
        self.tokens.push(TokenInfo::new(token, self.line))
    }
}
