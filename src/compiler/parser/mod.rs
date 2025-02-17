use super::{nodes::ast::Expression, CompilerCtx};
use crate::{compiler::lexer::token::Token, diagnostics::DiagnosticResult, FILE_EXTENSION};
use std::path::PathBuf;

mod expression;

use reader::TokenReader;
mod reader;

pub struct ParsedModule {
    expressions: Vec<Expression>,
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
            self.parse_tokens(&mut to_tokenize, path)?;
        }

        todo!()
    }
    pub(super) fn parse_tokens(
        &mut self,
        paths: &mut Vec<PathBuf>,
        current_path: PathBuf,
    ) -> DiagnosticResult<()> {
        let tokens = self.tokenize(&current_path)?;
        let mut tokens = TokenReader::new(tokens, current_path.clone());

        loop {
            if let Some(token) = tokens.next_if_expected(&vec![Token::Import, Token::EndOfFile]) {
                match token.raw {
                    Token::Import => {
                        let name = tokens.expect_identifier()?;
                        paths.push(current_path.join(name.raw))
                    }
                    Token::EndOfFile => break,
                    _ => continue,
                };
                continue;
            }

            let expression = tokens.parse_expression()?;

            // println!("{expression:?}");
        }

        todo!()
    }
}
