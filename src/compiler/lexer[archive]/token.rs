use std::ops::Range;

use crate::compiler::errors::Location;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    // Unkown,

    EndOfFile,
    Function,
    StartScope,
    EndScope,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Break,
    Pub,
    Import,
    Use,
    DoubleColon,
    Enum,
    Struct,
    Unsafe,
    Ampersand,
    SemiColon,
    Return,
    Dot,
    Underscore,
    Colon,
    Equals,
    Comma,
    Mutable,
    Variable,
    Give,
    ExclamationMark,

    If,
    ElseIf,
    Else,

    Plus,
    Minus,
    ForwardSlash,
    Asterisk,

    Loop,
    While,

    Compare,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,

    Boolean(bool),
    String(String),
    Integer(String),
    Float(String),
    Identifier(String),
}
impl Token {
    pub fn better_eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Boolean(_), Token::Boolean(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::Integer(_), Token::Integer(_)) => true,
            (Token::Float(_), Token::Float(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            _ => self == other,
        }
        // println!("{:?} == {:?} ({})", self, other, result);
        // result
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Token::*;
        write!(
            f,
            "{}",
            match self {
                ExclamationMark => "!",
                EndOfFile => "<eof>",
                Ampersand => "&",
                Function => "fn",
                StartScope => "{",
                EndScope => "}",
                OpenParen => "(",
                CloseParen => ")",
                OpenBracket => "[",
                CloseBracket => "]",
                Break => "break",
                Pub => "pub",
                Import => "import",
                Use => "use",
                DoubleColon => "::",
                Colon => ":",
                Enum => "enum",
                Struct => "struct",
                Unsafe => "unsafe",
                SemiColon => ";",
                Return => "return",
                Dot => ".",
                Underscore => "_",
                Equals => "=",
                Compare => "==",
                Comma => ",",
                Mutable => "mut",
                Variable => "let",
                Give => "give",
                If => "if",
                ElseIf => "elseif",
                Else => "else",
                Plus => "+",
                Minus => "-",
                Asterisk => "*",
                ForwardSlash => "/",
                Loop => "loop",
                While => "while",
                LessThan => "<",
                LessThanOrEquals => "<=",
                GreaterThan => ">",
                GreaterThanOrEquals => ">=",
                NotEquals => "!=",
                Boolean(_) => "bool",
                String(_) => "\"string\"",
                Integer(_) => "integer",
                Float(_) => "float",
                Identifier(_) => "Identifier",
            }
        )
    }
}


#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub location: Location,
}
impl TokenInfo {
    pub fn new(token: Token, lines: Range<usize>, columns: Range<usize>) -> Self {
        Self {
            token,
            location: Location::new(lines, columns),
        }
    }
}
