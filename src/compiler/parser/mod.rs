use super::{
    nodes::{
        ast::{Expression, Identifier, Parameter, RawExpression, Type},
        parser::{IntoParsingState, ParsingState, StartState},
    },
    CompilerCtx,
};
use crate::{
    common::position::{Position, PositionRange},
    compiler::lexer::token::Token,
    diagnostics::DiagnosticResult,
    FILE_EXTENSION,
};
use std::path::PathBuf;

mod common;
mod expression;
mod types;

use reader::TokenReader;
mod reader;

pub struct ParsedModule {
    expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct Parser {
    pub(super) tokens: TokenReader,
    pub(super) states: Vec<StartState>,
}
impl Parser {
    pub fn new(reader: TokenReader) -> Self {
        Self {
            tokens: reader,
            states: Vec::new(),
        }
    }
    pub fn path(&self) -> PathBuf {
        self.tokens.path()
    }
    pub fn push_state<T: IntoParsingState>(&mut self, state: T, position: PositionRange) {
        self.states
            .push(StartState::new(state.into_state(), position))
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
        let reader = TokenReader::new(tokens, current_path.clone());
        let mut parser = Parser::new(reader);

        loop {
            if let Some(token) = parser.next_if_expected(vec![Token::Import, Token::EndOfFile]) {
                match token.raw {
                    Token::Import => {
                        let name = parser.expect_identifier()?;
                        paths.push(current_path.join(name.raw))
                    }
                    Token::EndOfFile => break,
                    _ => continue,
                };
                continue;
            }

            let expression = parser.parse_expression()?;

            // println!("{expression:?}");
        }

        todo!()
    }
}
