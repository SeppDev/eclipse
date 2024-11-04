use std::path::PathBuf;

use super::{
    reader::{Char, Reader}, Token, TokenInfo, Tokens
};

pub fn tokenize(file_path: &PathBuf, source: String) -> Tokens {
    let mut reader = Reader::new(source);
    let mut cursor: usize = 0;

    loop {
        let mut chars = match reader.next(&cursor) {
            Some(chrs) => chrs,
            None => break,
        };
        let mut token: Option<Token> = None;

        loop {
            let string = match Char::to_string(&chars) {
                Some(s) => s,
                None => break,
            };

            match match_word(&string) {
                Some(t) => {
                    token = Some(t);
                    break;
                }
                None => {}
            }

            match is_number(&string) {
                Some(t) => {
                    token = Some(t);
                    break;
                }
                None => {}
            }

            match is_identifier(&string) {
                Some(t) => {
                    token = Some(t);
                    break;
                }
                None => {}
            }

            match string.as_str() {
                "\"" => {
                    let mut string = String::new();
                    loop {
                        cursor += 1;
                        let chr = match reader.get(&cursor) {
                            Some(c) => c,
                            None => break,
                        };
                        match chr.char {
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
                                    c => panic!("Unrecognized character: {:?}", c),
                                }
                            }
                            _ => string.push(chr.char),
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

    return Tokens::new(file_path.clone(), reader.tokens, reader.lines);
}

fn is_float(source: &String) -> Option<Token> {
    let mut dot = false;
    for chr in source.chars() {
        if chr.is_ascii_digit() {
            continue;
        }
        if chr == '.' && dot == false {
            dot = true;
            continue;
        }
        return None;
    }
    return Some(Token::Float(source.clone()));
}

fn is_number(source: &String) -> Option<Token> {
    let mut chrs = source.chars();
    if !chrs.next().unwrap().is_ascii_digit() {
        return None;
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
        return None;
    }
    return Some(Token::Integer(source.clone()));
}

fn is_valid_char(chr: char) -> bool {
    return chr == '_'
        || chr.is_ascii_lowercase()
        || chr.is_ascii_uppercase()
        || chr.is_ascii_digit();
}

fn is_identifier(source: &String) -> Option<Token> {
    let mut chars = source.chars();

    match chars.next() {
        Some(char) => {
            if char.is_ascii_digit() {
                return None;
            }
            if !is_valid_char(char) {
                return None;
            }
        }
        None => return None,
    }
    for char in chars {
        if is_valid_char(char) {
            continue;
        }
        return None;
    }
    return Some(Token::Identifier(source.to_string()));
}

fn match_word(word: &String) -> Option<Token> {
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
        "&" => Token::Reference,
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
        _ => return None,
    };

    return Some(token);
}
