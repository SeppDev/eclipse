use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::{Identifier, RawType, Type},
};

impl Tokens {
    pub fn parse_type(&mut self) -> CompileResult<Type> {
        if self
            .peek_expect_tokens(vec![Token::OpenParen], false)
            .is_some()
        {
            self.start()?;
            let mut tuple = Vec::new();
            loop {
                let new_type = self.parse_type()?;
                tuple.push(new_type);

                let result = self.expect_tokens(vec![Token::CloseParen, Token::Comma], false)?;
                match result.token {
                    Token::CloseParen => break,
                    Token::Comma => continue,
                    _ => panic!(),
                };
            }
            return Ok(self.create_located(RawType::Tuple(tuple)));
        }

        let info = self.expect_tokens(
            vec![
                Token::Ampersand,
                Token::Asterisk,
                Token::OpenBracket,
                Token::CloseBracket,
                Token::Identifier(String::new()),
            ],
            true,
        )?;

        let name = match info.token {
            Token::Ampersand => {
                let t = self.parse_type()?;
                return Ok(self.create_located(RawType::Reference(Box::new(t))));
            }
            Token::OpenBracket => {
                let data_type = self.parse_type()?;
                self.expect_tokens(vec![Token::SemiColon], false)?;

                let info = self.expect_tokens(vec![Token::Integer(String::new())], false)?;
                let count = match info.token {
                    Token::Integer(count) => count.parse::<usize>().unwrap(),
                    _ => panic!(),
                };

                let _ = self.expect_tokens(vec![Token::CloseBracket], false);
                return Ok(self.create_located(RawType::Array(count, Box::new(data_type))));
            }
            Token::OpenParen => todo!(),
            Token::Identifier(string) => Identifier {
                position: info.position,
                raw: string,
            },
            _ => return Ok(self.create_located(RawType::default())),
        };

        let raw = match name.raw.as_str() {
            "usize" => RawType::Usize,
            "isize" => RawType::Isize,
            "i64" => RawType::Int(64),
            "u64" => RawType::UInt(64),
            "i32" => RawType::Int(32),
            "u32" => RawType::UInt(32),
            "i16" => RawType::Int(16),
            "u16" => RawType::UInt(16),
            "i8" => RawType::Int(8),
            "u8" => RawType::UInt(8),
            "f32" => RawType::Float32,
            "f64" => RawType::Float64,
            "bool" => RawType::Boolean,
            "!" => RawType::Never,
            // _ => RawType::GetType(self.parse_path_root(name)?),
            raw => todo!("{raw}"),
        };
        return Ok(self.create_located(raw));
    }
}
