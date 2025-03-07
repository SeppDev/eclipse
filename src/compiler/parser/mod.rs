use super::{nodes::parser::ParserState, CompilerCtx};
use crate::{
    common::position::Located, compiler::lexer::token::Token, diagnostics::DiagnosticResult,
    FILE_EXTENSION,
};
use std::path::PathBuf;

mod common;
mod expression;
mod start;
mod types;

use reader::TokenReader;
mod reader;

pub struct Parser {
    pub tokens: TokenReader,
    pub stack: Vec<Located<ParserState>>,
}
impl Parser {
    pub fn new(reader: TokenReader) -> Self {
        Self {
            tokens: reader,
            stack: Vec::new(),
        }
    }
    pub fn path(&self) -> PathBuf {
        self.tokens.path()
    }
}
impl CompilerCtx {
    pub fn parse(&mut self) -> DiagnosticResult<()> {
        let mut to_tokenize = Vec::new();
        to_tokenize.push(PathBuf::from("src/main"));

        loop {
            let mut path = match to_tokenize.pop() {
                Some(p) => p,
                None => break,
            };
            path.set_extension(FILE_EXTENSION);
            let body = self.parse_tokens(&mut to_tokenize, path)?;
            println!("{}", ParserState::to_string_vec(&body));
            // println!("{:#?}", body);
        }

        Ok(())
    }
    pub(super) fn parse_tokens(
        &mut self,
        paths: &mut Vec<PathBuf>,
        current_path: PathBuf,
    ) -> DiagnosticResult<Vec<Located<ParserState>>> {
        let tokens = self.tokenize(&current_path)?;
        let reader = TokenReader::new(tokens, current_path.clone());
        let mut parser = Parser::new(reader);

        let mut body = Vec::new();

        loop {
            if parser.is_eof() {
                break;
            }

            if parser.next_if_eq(Token::Import)?.is_some() {
                let name = parser.expect_identifier()?;
                paths.push(current_path.join(name.raw));
                continue;
            }

            let state = parser.start_parse()?;

            body.push(state);
        }

        Ok(body)
    }
}
