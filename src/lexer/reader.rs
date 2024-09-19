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
    pub fn to_string(chars: &Vec<&Self>) -> Option<String> {
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
    pub tokens: Vec<TokenInfo>,
    pub lines: Vec<String>,
    chars: Vec<Char>,
}
impl Reader {
    pub fn new(source: String) -> Self {
        let source = source.replace("\r\n", "\n");
        let source = source.replace("\r", "\n");

        let split = source.chars();
        
        let mut vec = Vec::new();
        
        let mut lines = Vec::new();
        let mut line_string = String::new();
        let mut line: usize = 1;
        let mut column: usize = 0;
        
        for chr in split {
            match &chr {
                '\n' => {
                    lines.push(line_string);
                    line += 1;
                    column = 0;
                    line_string = String::new();
                }
                '\t' => {
                    column += 4;
                    line_string.push('\t')
                }
                ch => {
                    line_string.push(ch.clone());
                    column += chr.len_utf8();
                }
            }
            vec.push(Char {
                char: chr,
                column,
                line,
            });
        }
    

        vec.push(Char {
            char: ' ',
            column: 0,
            line: 0,
        });
        lines.push(line_string);

        Self {
            lines,
            tokens: Vec::new(),
            chars: vec,
            // index: 0
        }
    }
    pub fn get(&self, index: usize) -> Option<&Char> {
        return self.chars.get(index);
    }
    pub fn next(&self, cursor: &usize) -> Option<Vec<&Char>> {
        let mut chars = Vec::new();

        loop {
            let chr = match self.get(cursor + chars.len()) {
                Some(chr) => chr,
                None => break,
            };
            if chr.is_end() {
                if chars.len() == 0 {
                    return None;
                }
                break;
            }

            chars.push(chr);

            match chr.char {
                ' ' | '\t' | '\n' => {
                    if chars.len() > 1 {
                        chars.pop();
                    }
                    break;
                }
                _ => continue,
            }
            // let string = Char::to_string(chars);

            // match string.as_str() {
            //     "//" => {

            //     },
            //     _ => continue
            // }
        }

        return Some(chars);
    }
    pub fn push(&mut self, token: TokenInfo) {
        self.tokens.push(token);
    }
}
