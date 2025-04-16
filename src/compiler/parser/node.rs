use std::path::PathBuf;

use crate::{compiler::{lexer::token::Token, nodes::ast::Node, CompilerCtx}, diagnostics::DiagnosticResult, FILE_EXTENSION};

mod function;

use super::{reader::TokenReader, Parser};

impl Parser {
    pub fn parse_node(&mut self) -> DiagnosticResult<Node> {
        self.start();

        let token = self.next()?;
        match token.raw {

        }
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
            println!("{:#?}", body);
        }

        Ok(())
    }
    pub(super) fn parse_tokens(
        &mut self,
        paths: &mut Vec<PathBuf>,
        current_path: PathBuf,
    ) -> DiagnosticResult<()> {
        let tokens = self.tokenize(&current_path)?;
        let reader = TokenReader::new(tokens, current_path.clone());
        let mut parser = Parser::new(reader);

        loop {
            if parser.is_eof() {
                break;
            }

            if parser.next_if_eq(Token::Import)?.is_some() {
                let name = parser.expect_identifier()?;
                paths.push(current_path.join(name.raw));
                continue;
            }

            let expression = parser.parse_node();
            println!("{expression:?}");
        }

        Ok(())
    }
    
}
