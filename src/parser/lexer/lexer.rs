use std::str::Chars;

// #[derive(Debug)]
// pub enum TokenError {}
use super::{reader::Reader, token::{Operator, Token, TokenInfo}};
use eclipse::CompileError;



pub fn tokenize(source: String) -> Result<Vec<TokenInfo>, CompileError> {
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
        // println!("{:#?}", tokens.tokens);

        let mut string = match reader.next() {
            Some(token) => token,
            None => break,
        };
        
        loop {
            let length = string.len();

            if length == 0 {
                break;
            }

            match string.as_str() {
                "\n" => break,
                "\t" => break,
                "\r" => break,
                " " => break,
                _ => {}
            }

            let full_string = string.clone();
            for offset in 0..length {
                let new_string = full_string.get(..length - offset).unwrap();
                let token;
                match match_word(new_string) {
                    Some(t) => token = t,
                    None => match is_identifier(new_string) {
                        Some(t) => token = t,
                        None => match new_string {
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
                                    
                                    if ch == '"' {
                                        break;
                                    } else if ch == '\\' {
                                        offset += 2;
                                        match chars.next() {
                                            Some(chr) => match chr {
                                                'n' => token_string.push('\n'),
                                                '"' => token_string.push('"'),
                                                _ => todo!(),
                                            },
                                            None => todo!("Next character is required"),
                                        }
                                        continue;
                                    }
                                    offset += 1;
                                    token_string.push(ch);
                                }
                                token = Token::String(token_string);
                                string = string.get(offset..).unwrap().to_string();
                            }
                            _ => continue,
                        },
                    },
                }
                string = string.get(length - offset..).unwrap().to_string();
                reader.push(token);
                break;
            }
        }
    }

    reader.push(Token::EndOfFile);
    return Ok(reader.tokens);
}

fn is_integer(source: &String) -> Option<Token> {
    for char in source.chars() {
        if char.is_ascii_digit() {
            continue;
        }
        return None;
    }
    let string = source.to_string();
    return match string.parse::<isize>() {
        Ok(integer) => Some(Token::Integer(integer)),
        Err(a) => panic!("{:?}", a),
    };
}

fn is_valid_char(char: char) -> bool {
    return char == '_'
        || char.is_ascii_lowercase()
        || char.is_ascii_uppercase()
        || char.is_ascii_digit();
}

fn is_identifier(source: &str) -> Option<Token> {
    let mut chars = source.chars();

    match chars.next() {
        Some(char) => {
            if char.is_ascii_digit() {
                return is_integer(&source.to_string());
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
        "==" =>Token::Compare,
        "mut" => Token::Mutable,
        "&" => Token::Reference,
        "_" => Token::Underscore,
        "if" => Token::If,
        "else" => Token::Else,
        "+" => Token::Operator(Operator::Plus),
        "-" => Token::Operator(Operator::Minus),
        "*" => Token::Operator(Operator::Multiply),
        "/" => Token::Operator(Operator::Division),
        "pub" => Token::Pub,
        "return" => Token::Return,
        "let" => Token::Variable,
        "true" => Token::Boolean(true),
        "false" => Token::Boolean(false),
        "mod" => Token::Module,
        _ => return None,
    };
    return Some(token);
}
