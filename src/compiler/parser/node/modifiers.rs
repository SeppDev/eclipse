use crate::compiler::{
    diagnostics::{DiagnosticData, DiagnosticResult},
    lexer::token::TokenKind,
    nodes::ast::{Modifier, RawModifier, RawNode},
    parser::Parser,
};

use TokenKind::*;

impl Parser {
    pub fn expect_modifiers_list(&mut self) -> DiagnosticResult<Vec<Modifier>> {
        let start = self.start();
        let mut modifiers = Vec::with_capacity(3);

        while let Some(info) = self.next_if(|t| t.kind.is_modifier())? {
            let raw = match info.kind {
                Pub => RawModifier::Pub,
                Unsafe => RawModifier::Unsafe,
                Static => RawModifier::Static,
                Extern => RawModifier::Extern(self.expect_single(String)?.into()),
                _ => unreachable!(),
            };
            modifiers.push(self.located(raw, info.position.start))
        }

        let mut found: Vec<&Modifier> = Vec::with_capacity(modifiers.len());
        let mut duplicate = false;
        for modifier in &modifiers {
            for f in &found {
                if f == &modifier {
                    duplicate = true;
                    break;
                }
            }
            found.push(&modifier)
        }

        if duplicate {
            let current_position = self.last_position.end;
            return DiagnosticData::error()
                .title("Duplicate modifiers")
                .position(start.extend(current_position))
                .to_err();
        }

        Ok(modifiers)
    }
    pub fn expect_modifiers_node(&mut self) -> DiagnosticResult<RawNode> {
        Ok(RawNode::Modifiers(
            self.expect_modifiers_list()?,
            Box::new(self.expect_node()?),
        ))
    }
    pub fn expect_modifiers_expression(&mut self) -> DiagnosticResult<RawNode> {
        Ok(RawNode::Modifiers(
            self.expect_modifiers_list()?,
            Box::new(self.expect_expression()?),
        ))
    }
}
