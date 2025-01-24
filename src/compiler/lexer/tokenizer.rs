use crate::{
    common::location::{Position, PositionRange},
    compiler::{
        errors::{CompileCtx, CompileResult},
        lexer::reader::TokenKind,
    },
};

use super::{
    reader::{Char, Reader},
    Token, TokenInfo, Tokens,
};

const TAB_SIZE: usize = 4;

pub fn tokenize(ctx: &mut CompileCtx, source: String) -> CompileResult<Tokens> {
    let mut reader = Reader::new(source, TAB_SIZE);
    let mut tokens: Vec<TokenInfo> = Vec::new();

    loop {
        let token = match reader.next_string()? {
            Some(a) => a,
            None => break,
        };
        let info = handle_token(&mut reader, token)?;
        tokens.push(info)
    }

    let lines = reader.lines.len();

    tokens.push(TokenInfo::new(
        Token::EndOfFile,
        Position::new(lines, 0, 0).to_range(),
    ));
    ctx.set_lines(reader.lines);

    return Ok(Tokens::new(tokens, ctx.current_file_path.clone()));
}

fn handle_token(reader: &mut Reader, kind: TokenKind) -> CompileResult<TokenInfo> {
    match kind {
        TokenKind::Identifier(position, token) => {
            let token = match_token(&token).unwrap_or(Token::Identifier(token));
            return Ok(TokenInfo { position, token });
        }
        TokenKind::Integer(position, integer) => {
            let token = Token::Integer(integer);
            return Ok(TokenInfo { position, token });
        }
        TokenKind::String(position, string) => {
            let token = Token::String(string);
            return Ok(TokenInfo { position, token });
        }
        TokenKind::Float(position, base, decimal) => {
            let token = Token::Float(format!("{}.{}", base, decimal));
            return Ok(TokenInfo { position, token });
        }
        TokenKind::Punctuation(char) => {
            let second: Option<Char> = match reader.peek() {
                Some(second) => second
                    .char
                    .is_ascii_punctuation()
                    .then_some(reader.peek().unwrap().clone()),
                None => None,
            };

            match second {
                Some(second) => match match_token(&format!("{}{}", char.char, second.char)) {
                    Some(token) => {
                        reader.advance();
                        return Ok(TokenInfo {
                            token,
                            position: PositionRange::from(char.position, second.position),
                        });
                    }
                    None => {}
                },
                None => {}
            }

            let string = char.char.to_string();
            match match_token(&string) {
                Some(token) => {
                    return Ok(TokenInfo {
                        token,
                        position: char.position,
                    })
                }
                None => return Err(()),
            }
        }
    }
}

fn match_token(word: &String) -> Option<Token> {
    let token = match &word[..] {
        "func" => Token::Function,
        "if" => Token::If,
        "else" => Token::Else,
        "elseif" => Token::ElseIf,

        "mut" => Token::Mutable,
        "var" => Token::Variable,

        "true" => Token::Boolean(true),
        "false" => Token::Boolean(false),

        "pub" => Token::Pub,
        "import" => Token::Import,
        "use" => Token::Use,

        "unsafe" => Token::Unsafe,

        "enum" => Token::Enum,
        "struct" => Token::Struct,

        "return" => Token::Return,
        "result" => Token::Result,

        "loop" => Token::Loop,
        "while" => Token::While,
        "break" => Token::Break,
        "continue" => Token::Continue,

        "{" => Token::StartScope,
        "}" => Token::EndScope,
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
        "[" => Token::OpenBracket,
        "]" => Token::CloseBracket,

        "&" => Token::Ampersand,
        "_" => Token::Underscore,
        "!" => Token::ExclamationMark,

        "<<" => Token::LeftBitshift,
        ">>" => Token::RightBitshift,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Asterisk,
        "/" => Token::ForwardSlash,
        "%" => Token::Percent,
        "++" => Token::Increment,
        "--" => Token::Decrement,

        "." => Token::Dot,
        "," => Token::Comma,
        ";" => Token::SemiColon,
        ":" => Token::Colon,
        "::" => Token::DoubleColon,

        "+=" => Token::PlusEquals,
        "-=" => Token::SubtractEquals,
        "/=" => Token::DivideEquals,
        "*=" => Token::MultiplyEquals,
        "%=" => Token::PercentEquals,

        "<" => Token::LessThan,
        ">" => Token::GreaterThan,
        "<=" => Token::LessThanOrEquals,
        ">=" => Token::GreaterThanOrEquals,
        "!=" => Token::NotEquals,
        "==" => Token::Compare,
        "=" => Token::Equals,
        _ => return None,
    };

    return Some(token);
}
