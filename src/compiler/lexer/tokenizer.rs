use crate::compiler::{
    errors::{Location, MessageKind},
    path::Path,
};

use super::{
    reader::{Char, Reader},
    Token, TokenInfo, Tokens,
};

pub fn tokenize(relative_path: Path, source: String) -> Tokens {
    let mut reader = Reader::new(source);
    let mut cursor: usize = 0;

    loop {
        let mut chars = match reader.next(&cursor) {
            Some(chrs) => chrs,
            None => break,
        };
        let mut token: Option<Token> = None;

        loop {
            let mut string = match Char::to_string(&chars) {
                Some(s) => s,
                None => break,
            };

            string = match match_word(string) {
                Ok(t) => {
                    token = Some(t);
                    break;
                }
                Err(source) => source,
            };

            string = match is_number(string) {
                Ok(t) => {
                    token = Some(t);
                    break;
                }
                Err(source) => source,
            };

            string = match is_identifier(string) {
                Ok(t) => {
                    token = Some(t);
                    break;
                }
                Err(source) => source,
            };

            match string.as_str() {
                "\"" => {
                    let mut string = String::new();
                    loop {
                        cursor += 1;
                        let start_chr = match reader.get(&cursor) {
                            Some(c) => c,
                            None => break,
                        };
                        match start_chr.char {
                            '"' => break,
                            '\\' => {
                                cursor += 1;
                                let chr = match reader.get(&cursor) {
                                    Some(c) => c,
                                    None => break,
                                };
                                match chr.char {
                                    'n' => string.push('\n'),
                                    'r' => string.push('\r'),
                                    't' => string.push('\t'),
                                    '\\' => string.push('\\'),
                                    c => {
                                        file_messages.create(
                                            MessageKind::Error,
                                            Location::new(
                                                chr.line..chr.line,
                                                chr.column - 1..chr.column,
                                            ),
                                            format!("Unkown character escape: {:?}", c),
                                            "",
                                        );
                                    } //panic!("Unrecognized character: {:?}", c),
                                }
                            }
                            _ => string.push(start_chr.char),
                        }
                    }

                    token = Some(Token::String(string));
                    break;
                }
                _ => {}
            }

            chars.pop();
        }

        match token {
            Some(token) => {
                let start = chars.first().unwrap();
                let end = chars.last().unwrap();

                cursor += chars.len().max(1);
                reader.push(TokenInfo::new(
                    token,
                    start.line..end.line,
                    start.column..end.column + 1,
                ));
            }
            None => {
                cursor += chars.len().max(1);
            }
        }
    }
    
    let lines = reader.lines.len();
    reader.push(TokenInfo::new(Token::EndOfFile, lines..lines, 0..1));
    file_messages.set_lines(reader.lines);

    return Tokens::new(reader.tokens);
}

fn is_float(source: String) -> Result<Token, String> {
    let mut dot = false;
    for chr in source.chars() {
        if chr.is_ascii_digit() {
            continue;
        }
        if chr == '.' && dot == false {
            dot = true;
            continue;
        }
        return Err(source);
    }
    return Ok(Token::Float(source));
}

fn is_number(source: String) -> Result<Token, String> {
    let mut chrs = source.chars();
    if !chrs.next().unwrap().is_ascii_digit() {
        return Err(source);
    }

    for chr in chrs {
        if chr == '.' {
            return is_float(source);
        }
        if chr.is_ascii_digit() {
            continue;
        }
        if chr.is_alphabetic() {
            panic!("Failed to parse number!")
        }
        return Err(source);
    }
    return Ok(Token::Integer(source));
}

fn is_valid_char(chr: char) -> bool {
    return chr == '_'
        || chr.is_ascii_lowercase()
        || chr.is_ascii_uppercase()
        || chr.is_ascii_digit();
}

fn is_identifier(source: String) -> Result<Token, String> {
    let mut chars = source.chars();

    match chars.next() {
        Some(char) => {
            if char.is_ascii_digit() || !is_valid_char(char) {
                return Err(source);
            }
        }
        None => return Err(source),
    }
    for char in chars {
        if is_valid_char(char) {
            continue;
        }
        return Err(source);
    }
    return Ok(Token::Identifier(source));
}

fn match_word(word: String) -> Result<Token, String> {
    let token = match word.as_str() {
        "fn" => Token::Function,
        "{" => Token::StartScope,
        "}" => Token::EndScope,
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
        "[" => Token::OpenBracket,
        "]" => Token::CloseBracket,
        "," => Token::Comma,
        ":" => Token::Colon,
        ";" => Token::SemiColon,
        "=" => Token::Equals,
        "==" => Token::Compare,
        "mut" => Token::Mutable,
        "&" => Token::Ampersand,
        "_" => Token::Underscore,
        "if" => Token::If,
        "else" => Token::Else,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Asterisk,
        "/" => Token::ForwardSlash,
        "return" => Token::Return,
        "let" => Token::Variable,
        "true" => Token::Boolean(true),
        "false" => Token::Boolean(false),
        "pub" => Token::Pub,
        "import" => Token::Import,
        "use" => Token::Use,
        "." => Token::Dot,
        "::" => Token::DoubleColon,
        "unsafe" => Token::Unsafe,
        "enum" => Token::Enum,
        "struct" => Token::Struct,
        "give" => Token::Give,
        "loop" => Token::Loop,
        "while" => Token::While,
        "<" => Token::LessThan,
        ">" => Token::GreaterThan,
        "break" => Token::Break,
        _ => return Err(word),
    };

    return Ok(token);
}
