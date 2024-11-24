use crate::compiler::{
    errors::{CompileCtx, CompileResult, DebugInfo, Location},
    lexer::reader::TokenKind,
    path::Path,
};

use super::{
    reader::{Char, Reader},
    Token, TokenInfo, Tokens,
};

pub fn tokenize(
    debug: &mut CompileCtx,
    relative_file_path: Path,
    source: String,
) -> CompileResult<Tokens> {
    let mut reader = Reader::new(source, relative_file_path.clone());
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
    tokens.push(TokenInfo::new(Token::EndOfFile, lines..lines, 0..1));
    debug.set_lines(relative_file_path.clone(), reader.lines);

    return Ok(Tokens::new(tokens, relative_file_path));
}

fn handle_token(reader: &mut Reader, kind: TokenKind) -> CompileResult<TokenInfo> {
    match kind {
        TokenKind::Identifier(location, token) => {
            let token = match_token(&token).unwrap_or(Token::Identifier(token));
            return Ok(TokenInfo { location, token });
        }
        TokenKind::Integer(location, integer) => {
            let token = Token::Integer(integer);
            return Ok(TokenInfo { location, token });
        }
        TokenKind::String(location, string) => {
            let token = Token::String(string);
            return Ok(TokenInfo { location, token });
        }
        TokenKind::Float(location, base, decimal) => {
            let token = Token::Float(format!("{}.{}", base, decimal));
            return Ok(TokenInfo { location, token });
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
                            location: Location::new(
                                char.line..second.line,
                                char.columns.start..second.columns.end,
                            ),
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
                        location: Location::new(char.line..char.line, char.columns),
                    })
                }
                None => {
                    return Err(DebugInfo::new(
                        Location::single(char.line, char.columns.start),
                        reader.relative_file_path.clone(),
                        format!("Failed to find token: '{}'", string),
                        "",
                    ))
                }
            }
        }
    }
}

fn match_token(word: &String) -> Option<Token> {
    let token = match &word[..] {
        "func" => Token::Function,
        "mut" => Token::Mutable,
        "if" => Token::If,
        "else" => Token::Else,
        "elseif" => Token::ElseIf,
        "return" => Token::Return,
        "var" => Token::Variable,
        "true" => Token::Boolean(true),
        "false" => Token::Boolean(false),
        "pub" => Token::Pub,
        "import" => Token::Import,
        "use" => Token::Use,
        "unsafe" => Token::Unsafe,
        "enum" => Token::Enum,
        "struct" => Token::Struct,
        "give" => Token::Give,
        "loop" => Token::Loop,
        "while" => Token::While,
        "break" => Token::Break,

        "{" => Token::StartScope,
        "}" => Token::EndScope,
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
        "[" => Token::OpenBracket,
        "]" => Token::CloseBracket,

        "&" => Token::Ampersand,
        "_" => Token::Underscore,
        "!" => Token::ExclamationMark,

        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Asterisk,
        "/" => Token::ForwardSlash,

        "." => Token::Dot,
        "," => Token::Comma,
        ";" => Token::SemiColon,
        ":" => Token::Colon,
        "::" => Token::DoubleColon,

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
