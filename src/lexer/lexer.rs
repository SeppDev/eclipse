// #[derive(Debug)]
// pub enum TokenError {}

use super::token::Token;

pub fn tokenize(source: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut words = source.split(" ");
    let mut word = words.next().unwrap();

    loop {
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

    // println!("{:#?}", tokens.tokens);
    tokens.push(Token::EndOfFile);
    return Ok(tokens);
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
