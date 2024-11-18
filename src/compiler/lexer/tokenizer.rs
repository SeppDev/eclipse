use crate::compiler::{
    errors::{CompileMessages, CompileResult, Location},
    lexer::reader::{read_source, TokenKind},
    path::Path,
};

use super::{reader::Char, Token, TokenInfo, Tokens};

type TokenIter =
    std::iter::Peekable<std::vec::IntoIter<(crate::compiler::errors::Location, TokenKind)>>;

pub fn tokenize(
    compile_messages: &mut CompileMessages,
    relative_path: Path,
    source: String,
) -> CompileResult<Tokens> {
    use std::time::Instant;
    let strings = read_source(source)?;
    let mut strings: TokenIter = strings.into_iter().peekable();
    let mut tokens = Vec::new();

    loop {
        let (location, token) = match strings.next() {
            Some(result) => result,
            None => break,
        };

        tokens.push(handle_token(&mut strings, location, token)?)
    }

    // let lines = reader.lines.len();

    // reader.push(TokenInfo::new(Token::EndOfFile, lines..lines, 0..1));
    // compile_messages.set_lines(relative_path.clone(), reader.lines);

    // panic!("{:#?}", reader.tokens);

    // return Tokens::new(reader.tokens, relative_path);
    println!("{:#?}", tokens);
    return Ok(Tokens::new(tokens, relative_path));
}

fn handle_token(
    tokens: &mut TokenIter,
    mut location: Location,
    token: TokenKind,
) -> CompileResult<TokenInfo> {
    let token = match token {
        TokenKind::String(string) => Token::String(string),
        TokenKind::Integer(integer) => Token::Integer(integer),
        TokenKind::Float(int, decimal) => Token::Float(format!("{}.{}", int, decimal)),
        TokenKind::Identifier(string) => match_token(&string).unwrap_or(Token::Identifier(string)),
        TokenKind::Punctuation(char) => match expect_punctuation(tokens) {
            Some(second) => match match_token(&format!("{}{}", char, second.char)) {
                Some(t) => {
                    location.columns.end = second.column;
                    location.lines.end = second.line;
                    t
                }
                None => match match_token(&char.to_string()) {
                    Some(t) => t,
                    None => panic!("{}", char),
                },
            },
            None => match match_token(&char.to_string()) {
                Some(t) => t,
                None => panic!("{}", char),
            },
        },
    };
    return Ok(TokenInfo { token, location });
}

fn expect_punctuation(tokens: &mut TokenIter) -> Option<Char> {
    let (_, token) = match tokens.peek() {
        Some(result) => result,
        None => return None,
    };
    match token {
        TokenKind::Punctuation(_) => {
            let (location, token) = tokens.next().unwrap();
            let char = match token {
                TokenKind::Punctuation(char) => char,
                _ => panic!(),
            };
            return Some(Char {
                char,
                column: location.columns.start,
                line: location.lines.start,
            });
        }
        _ => return None,
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
