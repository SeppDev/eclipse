use std::{iter::Peekable, time::Instant, vec::IntoIter};

use super::TokenInfo;

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

#[derive(Debug)]
pub struct Reader {
    // pub lines: Vec<String>,
    chars: Peekable<IntoIter<Char>>,
}
impl Reader {
    pub fn new(source: String) -> Self {
        let mut output = Vec::with_capacity(source.len() + 1);
        let mut input = source.chars().into_iter();

        let mut lines: Vec<String> = Vec::new();
        let mut line_string: String = String::new();

        let mut line: usize = 1;
        let mut column: usize = 0;

        let s = Instant::now();
        loop {
            let char = match input.next() {
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
                    line_string.push_str("    ")
                }
                ch => {
                    line_string.push(ch);
                    column += 1
                }
            }

            output.push(Char {
                char,
                column,
                line,
            });
        }
        println!("chars: {:?}", s.elapsed());

        output.push(Char {
            char: ' ',
            column: 0,
            line: 0,
        });
        lines.push(line_string);

        Self {
            // lines,
            chars: output.into_iter().peekable(),
        }
    }
    pub fn next(&mut self) -> Option<Char> {
        self.chars.next()
    }
}
