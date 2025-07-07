use std::ops::Range;

use common::position::{Position, PositionRange};
use diagnostics::DiagnosticResult;
use kind::{LexerKind, LocatedString};
use reader::{Character, Reader};

use syntax::operators::{ArithmeticOperator, CompareOperator};
use token::{MAX_OPERATOR_WIDTH, Token, TokenKind, match_token};

pub mod token;

mod kind;
mod reader;

pub fn tokenize(source: &str) -> DiagnosticResult<Vec<Token>> {
    let mut reader = Reader::new(source);
    let mut tokens = Vec::new();

    loop {
        let kind = match reader.next()? {
            Some(k) => k,
            None => break,
        };
        let token = match kind {
            LexerKind::String(position) => {
                Token::new(position.raw, TokenKind::Text, position.position)
            }
            LexerKind::Character(position) => {
                Token::new(position.raw, TokenKind::Character, position.position)
            }
            LexerKind::Identifier(position) => {
                let kind = if let Some(kind) = match_token(&position.raw) {
                    kind
                } else {
                    TokenKind::Identifier
                };
                Token::new(position.raw, kind, position.position)
            }
            LexerKind::Integer(position) => {
                Token::new(position.raw, TokenKind::Integer, position.position)
            }
            LexerKind::Float(position) => {
                Token::new(position.raw, TokenKind::Float, position.position)
            }
            LexerKind::Operators(mut chars) => {
                let mut unkown = false;
                while chars.len() > 0 {
                    if unkown {
                        let char = chars.pop().unwrap();
                        tokens.push(Token::new(
                            char.raw.into(),
                            TokenKind::Unkown,
                            char.position,
                        ));
                        continue;
                    }

                    let len = chars.len().min(MAX_OPERATOR_WIDTH);

                    let mut is_final = true;
                    for i in 0..len {
                        let range = 0..len - i;
                        let string = chars_to_string(&chars, range.clone());

                        let kind = match match_token(&string.raw) {
                            Some(t) => t,
                            None => continue,
                        };

                        chars.drain(range);
                        tokens.push(Token::new(string.raw, kind, string.position));
                        is_final = false;
                        break;
                    }
                    unkown = is_final;
                }
                continue;
            }
        };
        tokens.push(token);
    }

    tokens.push(Token::new(
        "".into(),
        TokenKind::EndOfFile,
        Position::new(0, 0, 0).to_range(),
    ));
    Ok(tokens)
}

fn chars_to_string(chars: &Vec<Character>, range: Range<usize>) -> LocatedString {
    let slice = chars.get(range).unwrap();
    let body = slice.iter().map(|c| c.raw).collect::<String>();
    let start = slice.first().unwrap();
    let end = slice.last().unwrap();

    LocatedString::new(body, PositionRange::from(start.position, end.position))
}

pub struct ConversionError;

impl TryFrom<&TokenKind> for CompareOperator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        use TokenKind::*;

        let value = match value {
            NotEquals => CompareOperator::NotEquals,
            Compare => CompareOperator::Compare,
            GreaterThan => CompareOperator::GreaterThan,
            LessThan => CompareOperator::LessThan,
            LessThanOrEquals => CompareOperator::LessThanOrEquals,
            And => CompareOperator::And,
            Or => CompareOperator::Or,

            _ => return Err(ConversionError),
        };

        Ok(value)
    }
}
impl TryFrom<TokenKind> for CompareOperator {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&TokenKind> for ArithmeticOperator {
    type Error = ConversionError;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        use TokenKind::*;

        let value = match value {
            Plus => ArithmeticOperator::Plus,
            Minus => ArithmeticOperator::Subtract,
            Asterisk => ArithmeticOperator::Multiply,
            ForwardSlash => ArithmeticOperator::Division,
            Percent => ArithmeticOperator::Remainder,
            _ => return Err(ConversionError),
        };

        Ok(value)
    }
}
impl TryFrom<TokenKind> for ArithmeticOperator {
    type Error = ConversionError;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
