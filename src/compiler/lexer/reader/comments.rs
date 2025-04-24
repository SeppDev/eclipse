use super::LexerReader;

impl LexerReader {
    pub fn read_line_comment(&mut self) {
        loop {
            match self.advance_if(|c| c != &'\n') {
                Some(c) => c,
                None => break,
            };
        }
    }
    pub fn read_multi_line_comment(&mut self) {
        loop {
            let character = match self.advance() {
                Some(c) => c,
                None => break,
            };
            if character.raw == '*' && self.advance_if(|c| c == &'/').is_some() {
                break;
            }
        }
    }
}
