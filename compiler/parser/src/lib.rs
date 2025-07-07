use ::common::position::PositionRange;
use diagnostics::DiagnosticResult;
use lexer::token::Token;
use syntax::ast;

mod common;
mod imports;
mod node;

pub fn parse(mut tokens: Vec<Token>) -> DiagnosticResult<Vec<ast::Node>> {
    tokens.reverse();

    let mut parser = Parser {
        tokens,
        last_position: PositionRange::default(),
    };

    let mut nodes = Vec::new();

    loop {
        if parser.is_eof() {
            break;
        }

        let node = parser.top_level_expect()?;
        nodes.push(node);
    }

    Ok(nodes)
}

struct Parser {
    tokens: Vec<Token>,
    last_position: PositionRange,
}
