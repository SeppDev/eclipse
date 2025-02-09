use super::{context::CompileCtx, lexer::token::TokenInfo};
use crate::common::{errors::CompileResult, files::FILE_EXTENSION};
use std::path::PathBuf;

mod handle;

impl CompileCtx {
    pub fn parse(&mut self) -> CompileResult<()> {
        let mut to_tokenize = Vec::new();
        to_tokenize.push(PathBuf::from("src/main"));

        loop {
            let path = match to_tokenize.pop() {
                Some(p) => p,
                None => break,
            };
            self.parse_tokens(&mut to_tokenize, path)?;
        }

        todo!()
    }
    pub fn parse_file(&self, mut relative_path: PathBuf) -> CompileResult<Vec<TokenInfo>> {
        relative_path.set_extension(FILE_EXTENSION);
        self.tokenize(&relative_path)
    }
}
