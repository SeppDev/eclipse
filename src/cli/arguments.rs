use std::{env::Args, iter::Peekable, path::PathBuf};

use crate::common::exit::exit;

pub enum Argument {
    Value(String),
    KeyValue(String, String),
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
        if self.current_args.next_if_eq("=").is_some() {
            let value = match self.current_args.next() {
                Some(v) => v,
                None => exit(format!("Expected value for key: '{key}'"))
            };
            return Some(Argument::KeyValue(key, value))
        }
        
        Some(Argument::Value(key))
    }
}
