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
    // Operator(Operator),
    Boolean(bool),
    String(String),
    Integer(isize),
    Identifier(String),
}