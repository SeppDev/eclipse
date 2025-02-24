use super::{nodes::ast::Node, CompilerCtx};
use crate::{compiler::lexer::token::Token, diagnostics::DiagnosticResult, FILE_EXTENSION};
use std::path::PathBuf;

mod common;
mod expression;
mod types;

use reader::TokenReader;
mod reader;

pub struct ParsedModule {
    expressions: Vec<Node>,
}

#[derive(Debug)]
pub struct Parser {
    pub(super) tokens: TokenReader,
}
impl Parser {
    pub fn new(reader: TokenReader) -> Self {
        Self { tokens: reader }
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
            println!("{body:#?}");
        }

        Ok(())
    }
    pub(super) fn parse_tokens(
        &mut self,
        paths: &mut Vec<PathBuf>,
        current_path: PathBuf,
    ) -> DiagnosticResult<Vec<Node>> {
        let tokens = self.tokenize(&current_path)?;
        let reader = TokenReader::new(tokens, current_path.clone());
        let mut parser = Parser::new(reader);

        let mut body = Vec::new();

        loop {
            if let Some(token) = parser.next_if_expected(vec![Token::Import, Token::EndOfFile]) {
                match token.raw {
                    Token::Import => {
                        let name = parser.expect_identifier()?;
                        paths.push(current_path.join(name.raw))
                    }
                    Token::EndOfFile => break,
                    _ => unreachable!(),
                };
                continue;
            }

            let node = parser.parse_node()?;

            body.push(node);
        }

        Ok(body)
    }
}
