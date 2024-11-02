use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[allow(dead_code)]
pub enum Token {
    #[default]
    None,
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
    Reference,
    SemiColon,
    Return,
    Dot,
    Underscore,
    Colon,
    Equals,
    Compare,
    Comma,
    Mutable,
    Variable,
    Give,
    If,
    Else,
    Plus,
    Minus,
    ForwardSlash,
    Asterisk,
    Loop,
    While,
    LessThan,
    GreaterThan,
    Boolean(bool),
    String(String),
    Integer(String),
    Float(String),
    Identifier(String),
}

#[derive(Debug)]
pub struct TokenInfo {
    pub token: Token,
    pub lines: Range<usize>,
    pub columns: Range<usize>
}
impl TokenInfo {
    pub fn new(token: Token, lines: Range<usize>, columns: Range<usize>) -> Self {
        Self {
            token,
            lines,
            columns
        }
    }
}
