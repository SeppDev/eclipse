use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Token::*;
        write!(
            f,
            "{}",
            match self {
                EndOfFile => "<eof>",
                Reference => "&",
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
                Else => "else",
                Plus => "+",
                Minus => "-",
                Asterisk => "*",
                ForwardSlash => "/",
                Loop => "loop",
                While => "while",
                LessThan => "<",
                GreaterThan => ">",
                Boolean(_) => "bool",
                String(value) => value.as_str(),
                Integer(value) => value.as_str(),
                Float(value) => value.as_str(),
                Identifier(value) => value.as_str(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub lines: Range<usize>,
    pub columns: Range<usize>,
}
impl Location {
    pub fn new(lines: Range<usize>, columns: Range<usize>) -> Self {
        Self { lines, columns }
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
