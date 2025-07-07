use common::position::LocatedAt;
use diagnostics::{DiagnosticData, DiagnosticResult};
use lexer::token::TokenKind::*;
use syntax::ast::{RawType, Type};

use crate::Parser;

impl Parser {
    pub fn expect_type(&mut self) -> DiagnosticResult<Type> {
        let info = match self.expect(&vec![Identifier, Ampersand, OpenBracket, OpenParen]) {
            Ok(i) => i,
            Err(_) => {
                let peeked = self.peek();
                return DiagnosticData::error()
                    .title(format!("Expected type but, got: {:?}", &peeked.kind))
                    .position(peeked.position)
                    .to_err();
            }
        };

        let raw: RawType = match info.kind {
            OpenParen => {
                let mut list = Vec::new();

                while self.next_if_eq(CloseParen)?.is_none() {
                    let data_type = self.expect_type()?;
                    list.push(data_type);
                }

                RawType::Tuple(list)
            }
            OpenBracket => {
                let data_type = self.expect_type()?;
                if self.next_if_eq(SemiColon)?.is_some() {
                    let amount = self.expect_identifier()?.into();
                    self.expect_single(CloseBracket)?;
                    RawType::Array(Box::new(data_type), amount)
                } else {
                    self.expect_single(CloseBracket)?;
                    RawType::Slice(Box::new(data_type))
                }
            }
            Ampersand if self.peek().kind == Apostrophe => {
                self.next()?;
                let lifetime = Some(self.expect_identifier()?.into());
                let data_type = self.expect_type()?;
                RawType::Ref(lifetime, Box::new(data_type))
            }
            Ampersand if self.peek().kind == Mutable => {
                self.next()?;
                RawType::RefMut(None, Box::new(self.expect_type()?))
            }
            Ampersand => RawType::Ref(None, Box::new(self.expect_type()?)),

            _ => match info.string.as_str() {
                "i64" => RawType::Int(64),
                "i32" => RawType::Int(32),
                "i16" => RawType::Int(16),
                "i8" => RawType::Int(8),

                "u64" => RawType::Int(64),
                "u32" => RawType::UInt(32),
                "u16" => RawType::UInt(16),
                "u8" => RawType::UInt(8),

                "f32" => RawType::Float32,
                "f64" => RawType::Float64,

                "void" => RawType::Void,
                "bool" => RawType::Boolean,
                "never" => RawType::Never,
                "str" => RawType::String,
                "char" => RawType::Char,
                "isize" => RawType::ISize,
                "usize" => RawType::USize,

                "Self" => RawType::SelfType,
                _ if self.peek().kind == DoubleColon => {
                    let mut path = Vec::new();
                    while self.next_if_eq(DoubleColon)?.is_some() {
                        let identifier = self.expect_identifier()?.into();
                        path.push(identifier);
                    }
                    RawType::Other(path)
                }
                _ => RawType::Other(vec![LocatedAt::<String>::new(info.string, info.position)]),
            },
        };
        Ok(Type::new(raw, info.position))
    }
}
