use std::collections::HashMap;
mod value;
pub use value::*;

mod stringify;

pub type JSON = Value;

pub fn new() -> Value {
    Value::Object(HashMap::new())
}

pub fn from_str<Content: ToString>(content: Content) -> Value {
    let mut content = content.to_string().drain(..).collect::<Vec<char>>();
    content.reverse();

    let body = String::from_iter(content.iter());
    let mut object = Object { body };

    return object.parse();
}

pub(super) struct Object {
    pub body: String,
}

impl Object {
    pub(super) fn parse(&mut self) -> Value {
        self.trim();
        loop {
            let char: char = self.next().unwrap();
            return match char {
                '{' => self.parse_map(),
                '[' => self.parse_array(),
                '"' => Value::String(self.parse_until_delimiter_string('"')),
                '\'' => Value::String(self.parse_until_delimiter_string('\'')),
                '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.parse_number(Some(char))
                }
                ' ' => continue,
                _ => self.parse_literal(Some(char)),
            };
        }
    }
    pub(super) fn next(&mut self) -> Option<char> {
        let char = self.body.pop();
        println!("{char:?}");
        return char;
    }
}

impl Object {
    pub(super) fn parse_literal(&mut self, start: Option<char>) -> Value {
        let mut literal = String::new();
        if let Some(char) = start {
            self.body.push(char);
        }

        loop {
            let char: char = self.next().unwrap();
            if !char.is_ascii_alphabetic() {
                break;
            }
            literal.push(char);
        }

        return match &literal[..] {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            "null" => Value::Null,
            _ if literal.len() == 0 => Value::Null,
            _ => Value::Literal(literal),
        };
    }
}

impl Object {
    pub(super) fn parse_number(&mut self, start: Option<char>) -> Value {
        let mut number = String::new();
        if let Some(char) = start {
            number.push(char);
        }

        loop {
            let char: char = self.next().unwrap();
            match char {
                '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    number.push(char)
                }
                _ => break,
            }
        }

        return Value::Number(value::Number(number));
    }
}

impl Object {
    pub(super) fn parse_map(&mut self) -> Value {
        let mut map = HashMap::new();

        loop {
            self.trim();

            let char: char = match self.next() {
                Some(c) => c,
                None => break,
            };

            let key = match char {
                '"' => self.parse_until_delimiter_string('"'),
                '\'' => self.parse_until_delimiter_string('\''),
                '}' => break,
                ',' => continue,
                _ => panic!("Expected string, got: {char:?}"),
            };

            self.parse_until_delimiter(':');

            let value = self.parse();

            println!("{key}: {value:?}");
            map.insert(key, value);
        }

        return Value::Object(map);
    }
}

impl Object {
    pub(super) fn parse_array(&mut self) -> Value {
        let mut array = Vec::new();

        loop {
            self.trim();

            let char: char = match self.next() {
                Some(c) => c,
                None => break,
            };

            match char {
                ']' => break,
                ',' => continue,
                _ => self.body.push(char),
            };

            let value = self.parse();
            println!("{value:?}");
            array.push(value);

            self.trim();
        }

        return Value::Array(array);
    }
}

impl Object {
    pub(super) fn trim(&mut self) {
        loop {
            let char: char = match self.next() {
                Some(c) => c,
                None => break,
            };

            if char.is_whitespace() {
                continue;
            }

            self.body.push(char);
            break;
        }
    }
}

impl Object {
    pub(super) fn parse_until_delimiter(&mut self, delimiter: char) {
        loop {
            let char: char = self.next().unwrap();
            if char == delimiter {
                break;
            }
        }
    }
}

impl Object {
    pub(super) fn parse_until_delimiter_string(&mut self, delimiter: char) -> String {
        let mut content = String::new();

        loop {
            let char: char = self.next().unwrap();
            if char == delimiter {
                break;
            }

            content.push(char);
        }

        return content;
    }
}
