use crate::compiler::{errors::CompileMessages, path::Path};

use super::{reader::Reader, Token, Tokens};

pub fn tokenize(
    compile_messages: &mut CompileMessages,
    relative_path: Path,
    source: String,
) -> Tokens {
    use std::time::Instant;
    let lexer_time = Instant::now();
    let mut reader = Reader::new(source);

    loop {
        let (token, kind, location) = match reader.next_string() {
            Ok(result) => match result {
                Some((t, k, l)) => (t, k, l),
                None => break,
            },
            Err(()) => panic!(),
        };

        println!("{} {:?} {}", token, kind, location)
    }

    // let lines = reader.lines.len();

    // reader.push(TokenInfo::new(Token::EndOfFile, lines..lines, 0..1));
    // compile_messages.set_lines(relative_path.clone(), reader.lines);

    // panic!("{:#?}", reader.tokens);

    // return Tokens::new(reader.tokens, relative_path);
    println!("lexer: {:?}", lexer_time.elapsed());
    todo!()
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

fn match_token(word: String) -> Result<Token, String> {
    let token = match &word[..] {
        "func" => Token::Function,
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
        "elseif" => Token::ElseIf,
        "!" => Token::ExclamationMark,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Asterisk,
        "/" => Token::ForwardSlash,
        "return" => Token::Return,
        "var" => Token::Variable,
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
        "<=" => Token::LessThanOrEquals,
        ">" => Token::GreaterThan,
        ">=" => Token::GreaterThanOrEquals,
        "!=" => Token::NotEquals,
        "break" => Token::Break,
        _ => return Err(word),
    };

    return Ok(token);
}
