use super::{lexer::token::TokenInfo, nodes::ast::Expression, CompilerCtx};
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
            let token = tokens.advance_if(&vec![Token::Import, Token::EndOfFile]);

            let expression = match token {
                Some(token) => match token.raw {
                    Token::Import => {
                        let name = tokens.expect_identifier()?.raw;

                        continue;
                    }
                    Token::EndOfFile => break,
                    _ => todo!(),
                },
                None => tokens.parse_expression()?,
            };

            println!("{expression:?}");
        }

        todo!()
    }
}
