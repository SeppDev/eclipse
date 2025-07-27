use std::{env::Args, iter::Peekable, path::PathBuf};

use common::exit::exit;

pub enum Argument {
    Value(String),
    KeyValue(String, String),
}
impl Argument {
    pub fn into_value(self) -> String {
        if let Self::Value(value) = self {
            value
        } else {
            exit("Expected value got 'key:value'")
        }
    }
    pub fn into_key_value(self) -> (String, String) {
        if let Self::KeyValue(key, value) = self {
            (key, value)
        } else {
            exit("Expected value got 'value'")
        }
    }
}

pub struct Arguments {
    current_args: Peekable<Args>,
    current_dir: PathBuf,
}
impl Arguments {
    pub fn new() -> Self {
        let mut current_args = std::env::args().peekable();
        let current_dir = std::env::current_dir().unwrap();

        let _ = current_args.next();

        Self {
            current_dir,
            current_args,
        }
    }
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
    pub fn next_argument(&mut self) -> Option<Argument> {
        let key = self.current_args.next()?;
        if let Some((key, value)) = key.split_once('=') {
            return Some(Argument::KeyValue(key.to_string(), value.to_string()));
        }

        Some(Argument::Value(key))
    }
    pub fn expect_argument(&mut self, message: Option<&'static str>) -> Argument {
        match self.next_argument() {
            Some(n) => n,
            None => exit(message.unwrap_or("Expected argument")),
        }
    }
}
