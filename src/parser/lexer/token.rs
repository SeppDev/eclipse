#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum Token {
    EndOfFile,
    
    Function,
    StartScope,
    EndScope,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,

    Module,
    Use,
    Pub,
    Reference,
    SemiColon,
    Return,
    Underscore,
    Colon,
    Equals,
    Compare,
    Comma,
    Mutable,
    Variable,
    If,
    Else,
    Operator(Operator),
    Boolean(bool),
    String(String),
    Integer(isize),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    Multiply
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize
}
impl TokenInfo {
    pub fn new(token: Token, line: usize, column: usize) -> Self {
        Self {
            token,
            column,
            line
        }
    }
}