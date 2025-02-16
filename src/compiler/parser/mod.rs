use super::{lexer::token::TokenInfo, CompilerCtx};
use crate::{compiler::lexer::token::Token, diagnostics::DiagnosticResult, FILE_EXTENSION};
use std::path::PathBuf;

use reader::TokenReader;
mod reader;

impl CompilerCtx {
    pub fn parse(&mut self) -> DiagnosticResult<()> {
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
    pub fn parse_file(&mut self, mut relative_path: PathBuf) -> DiagnosticResult<Vec<TokenInfo>> {
        relative_path.set_extension(FILE_EXTENSION);
        self.tokenize(&relative_path)
    }
    pub(super) fn parse_tokens(
        &mut self,
        paths: &mut Vec<PathBuf>,
        mut current_path: PathBuf,
    ) -> DiagnosticResult<()> {
        current_path.set_extension(FILE_EXTENSION);
        
        let tokens = self.tokenize(&current_path)?.into_iter().peekable();
        let mut tokens = TokenReader::new(tokens, current_path.clone());

        loop {
            let token = tokens.expect(&vec![Token::Import, Token::Function, Token::EndOfFile])?;
            match token.raw {
                Token::Function => {
                    let name = tokens.expect_identifier()?;
                    todo!("{name:#?}");
                }
                Token::EndOfFile => break,
                _ => {}
            }
        }

        todo!()
    }
}
