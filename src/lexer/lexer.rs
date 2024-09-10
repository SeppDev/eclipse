use std::{path::PathBuf, str::Chars};

// #[derive(Debug)]
// pub enum TokenError {}
use super::{reader::Reader, token::Token, TokensGroup};

pub fn tokenize(source: String, relative_path: PathBuf) -> Result<TokensGroup, (String, Reader)> {
    let mut reader;
    {
        let split = source.chars();
        let mut vec: Vec<char> = Vec::new();
        for char in split {
            vec.push(char);
        }

        reader = Reader::new(vec);
    }
    // let mut word = tokens.next().unwrap();

    loop {
        let mut string = match reader.next() {
            Some(token) => token,
            None => break,
        };

        loop {
            let length = string.len();

            if length == 0 {
                break;
            }
            if string == "//" {
                loop {
                    match reader.next() {
                        Some(str) => {
                            if str == "\n" {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                break;
            }

            match string.as_str() {
                "\n" => break,
                "\t" => break,
                "\r" => break,
                " " => break,
                _ => {}
            }

            for offset in 0..length {
                let match_string = string.get(..length - offset).unwrap();
                let token;

                if match_string == "//" {
                    string = match_string.to_string();
                    break;
                }

                match match_word(match_string) {
                    Some(t) => token = Some(t),
                    None => match is_identifier(match_string) {
                        Ok(t) => match t {
                            Some(t) => token = Some(t),
                            None => match match_string {
                                "\"" => {
                                    let mut token_string = String::new();
                                    string = string.get(1..).unwrap().to_string();

                                    let mut offset: usize = 0;
                                    let mut chars: Chars = string.chars();

                                    loop {
                                        let ch = match chars.next() {
                                            Some(ch) => ch,
                                            None => {
                                                offset = 0;
                                                string = reader.next().unwrap();
                                                chars = string.chars();
                                                continue;
                                            }
                                        };

                                        // println!("{:?}", token_string);

                                        if ch == '"' {
                                            break;
                                        } else if ch == '\\' {
                                            offset += 2;
                                            match chars.next() {
                                                Some(chr) => match chr {
                                                    'n' => token_string.push('\n'),
                                                    't' => token_string.push('\t'),
                                                    'r' => token_string.push('\r'),
                                                    '\\' => token_string.push('\\'),
                                                    '"' => token_string.push('"'),
                                                    _ => todo!(),
                                                },
                                                None => {
                                                    return Err((
                                                        String::from("Expected n,t"),
                                                        reader,
                                                    ))
                                                }
                                            }
                                            continue;
                                        }
                                        offset += 1;
                                        token_string.push(ch);
                                    }
                                    token = Some(Token::String(token_string));
                                    string = string.get(offset..).unwrap().to_string();
                                }
                                _ => {
                                    assert!(match_string.len() > 1);
                                    continue;
                                }
                            },
                        },
                        Err(error) => return Err((error, reader)),
                    },
                }

                string = match string.get(length - offset..) {
                    Some(s) => s.to_string(),
                    None => return Err((String::from("Unsuported token"), reader)),
                };

                match token {
                    Some(t) => {
                        reader.push(t);
                        break;
                    }
                    None => panic!(),
                }
            }
        }
    }

    reader.push(Token::EndOfFile);
    return Ok(TokensGroup::new(reader.tokens, relative_path));
}

fn is_integer(source: &String) -> Result<Option<Token>, String> {
    for char in source.chars() {
        if char.is_ascii_digit() {
            continue;
        }
        return Ok(None);
    }
    let string = source.to_string();
    return match string.parse::<usize>() {
        Ok(integer) => Ok(Some(Token::Integer(integer))),
        Err(a) => Err(format!("{:?}", a)),
    };
}

fn is_valid_char(char: char) -> bool {
    return char == '_'
        || char.is_ascii_lowercase()
        || char.is_ascii_uppercase()
        || char.is_ascii_digit();
}

fn is_identifier(source: &str) -> Result<Option<Token>, String> {
    let mut chars = source.chars();

    match chars.next() {
        Some(char) => {
            if char.is_ascii_digit() {
                return is_integer(&source.to_string());
            }
            if !is_valid_char(char) {
                return Ok(None);
            }
        }
        None => return Ok(None),
    }
    for char in chars {
        if is_valid_char(char) {
            continue;
        }
        return Ok(None);
    }
    return Ok(Some(Token::Identifier(source.to_string())));
}

fn match_word(word: &str) -> Option<Token> {
    let token = match word {
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
        "/" => Token::Slash,
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
        _ => return None,
    };

    return Some(token);
}
