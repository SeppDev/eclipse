use std::collections::HashMap;
mod value;
pub use value::*;
mod common;

mod stringify;

pub trait ToJson {
    fn to_json(self) -> JSONObject;
}

pub type JSON = JSONObject;

pub fn new() -> JSONObject {
    JSONObject::new()
}

pub fn from_str<Content: ToString>(content: Content) -> JSONObject {
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
    pub(super) fn parse(&mut self) -> JSONObject {
        self.trim();
        loop {
            let char: char = self.next().unwrap();
            return match char {
                '{' => self.parse_map(),
                '[' => self.parse_array(),
                '"' => JSONObject::String(self.parse_until_delimiter_string('"')),
                '\'' => JSONObject::String(self.parse_until_delimiter_string('\'')),
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
    pub(super) fn parse_literal(&mut self, start: Option<char>) -> JSONObject {
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
            "true" => JSONObject::Bool(true),
            "false" => JSONObject::Bool(false),
            "null" => JSONObject::Null,
            _ if literal.len() == 0 => JSONObject::Null,
            _ => JSONObject::Literal(literal),
        };
    }
}

impl Object {
    pub(super) fn parse_number(&mut self, start: Option<char>) -> JSONObject {
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

        return JSONObject::Number(value::Number(number));
    }
}

impl Object {
    pub(super) fn parse_map(&mut self) -> JSONObject {
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

        return JSONObject::Object(map);
    }
}

impl Object {
    pub(super) fn parse_array(&mut self) -> JSONObject {
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

        return JSONObject::Array(array);
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

#[macro_export]
macro_rules! json {
    ( $value:expr ) => {
        {
            $crate::lsp::json::to_json_trait($value)
        }
    };

    ( $($k:ident : $v:expr),* ) => {
        {
            let mut obj = $crate::lsp::json::JSONObject::new();

            $(
                // obj.insert(stringify!($k).to_string(), $crate::lsp::json::json!($v));
                obj.insert(stringify!($k).to_string(), $crate::lsp::json::to_json_trait($v));
            )*

            obj
        }
    };
}

pub(crate) fn to_json_trait<T: ToJson>(value: T) -> JSONObject {
    value.to_json()
}

impl ToJson for bool {
    fn to_json(self) -> JSONObject {
        JSONObject::Bool(self)
    }
}
impl ToJson for String {
    fn to_json(self) -> JSONObject {
        JSONObject::String(self)
    }
}
impl ToJson for &str {
    fn to_json(self) -> JSONObject {
        JSONObject::String(self.to_string())
    }
}

impl ToJson for usize {
    fn to_json(self) -> JSONObject {
        JSONObject::Number(Number(self.to_string()))
    }
}
