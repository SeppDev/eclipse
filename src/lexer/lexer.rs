use std::str::Chars;

// #[derive(Debug)]
// pub enum TokenError {}
use super::token::{Token, TokenInfo};
use eclipse::BuildError;
use reader::Tokens;

mod reader;



pub fn tokenize(source: String) -> Result<Vec<TokenInfo>, BuildError> {
    let mut tokens;
    {
        let split = source.chars();
        let mut vec: Vec<char> = Vec::new();
        for char in split {
            vec.push(char);
        }

        tokens = Tokens::new(vec);
    }
    // let mut word = tokens.next().unwrap();

    loop {
        // println!("{:#?}", tokens.tokens);

        let mut string = match tokens.next() {
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
                                    offset += 1;
                                    let ch = match chars.next() {
                                        Some(ch) => ch,
                                        None => {
                                            offset = 0;
                                            string = tokens.next().unwrap();
                                            chars = string.chars();
                                            continue;
                                        }
                                    };
                                    
                                    if ch == '"' {
                                        break;
                                    } else if ch == '\\' {
                                        offset += 1;
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
                tokens.push(token);
                break;
            }
        }
    }

    /* loop {
        if word.is_empty() {
            word = match words.next() {
                Some(a) => a,
                None => break,
            };
            continue;
        }

        {
            let firstchar = word.chars().next().unwrap();
            let string = firstchar.to_string();
            let str = string.as_str();

            if match str {
                "\n" => true,
                "\r" => true,
                "\t" => true,
                _ => false,
            } {
                word = word.get(1..).unwrap_or_default();
                continue;
            }
            if firstchar == '"' {
                let mut chars = word.chars();
                chars.next();

                let mut string = String::new();
                loop {
                    let char = match chars.next() {
                        Some(a) => a,
                        None => {
                            word = words.next().unwrap();
                            chars = word.chars();
                            string.push_str(" ");
                            continue;
                        }
                    };
                    if char == '"' {
                        break;
                    }
                    string.push(char);
                }
                tokens.push(Token::String(string));

                let length = word.len();
                let count = chars.count();
                word = word.get(length - count..).unwrap_or_default();
                continue;
            }
        }

        let length: usize = word.len();
        let mut offset: usize = 1;

        for i in 0..length {
            let substring = match word.get(..(length - i)) {
                Some(a) => a,
                None => break,
            };
            match match_word(substring) {
                Some(token) => {
                    tokens.push(token);
                    offset = substring.len();
                    break;
                }
                None => match is_identifier(substring) {
                    Some(token) => {
                        tokens.push(token);
                        offset = substring.len();
                        break;
                    }
                    None => {}
                },
            }
        }

        if !word.is_empty() {
            if offset == 0 {
                panic!("No token found: {:?}", word);
            };
            word = word.get(offset..).unwrap_or_default();
            continue;
        }
    }
    */

    // println!("{:#?}", tokens.tokens);
    tokens.push(Token::EndOfFile);
    return Ok(tokens.tokens);
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
    match word {
        "fn" => Some(Token::Function),
        "{" => Some(Token::StartScope),
        "}" => Some(Token::EndScope),
        "(" => Some(Token::OpenParen),
        ")" => Some(Token::CloseParen),
        "[" => Some(Token::OpenBracket),
        "]" => Some(Token::CloseBracket),
        "," => Some(Token::Comma),
        ":" => Some(Token::Colon),
        ";" => Some(Token::SemiColon),
        "=" => Some(Token::Equals),
        "==" => Some(Token::Compare),
        "mut" => Some(Token::Mutable),
        "&" => Some(Token::Reference),
        "_" => Some(Token::Underscore),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        // "+" => Some(Token::Operator(Operator::Plus)),
        // "-" => Some(Token::Operator(Operator::Minus)),
        // "*" => Some(Token::Operator(Operator::Multiply)),
        // "/" => Some(Token::Operator(Operator::Division)),
        "return" => Some(Token::Return),
        "let" => Some(Token::Variable),
        "true" => Some(Token::Boolean(true)),
        "false" => Some(Token::Boolean(false)),
        _ => None,
    }
}
