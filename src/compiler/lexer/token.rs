use crate::common::position::Located;

pub const MAX_OPERATOR_WIDTH: usize = 2;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    EndOfFile,
    Function,
    OpenBlock,
    CloseBlock,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
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
    Result,
    Dot,
    Underscore,
    Colon,
    Equals,
    Comma,
    Mutable,
    Variable,
    ExclamationMark,
    Arrow,
    FatArrow,

    If,
    ElseIf,
    Else,

    LeftBitshift,
    RightBitshift,
    Plus,
    Minus,
    ForwardSlash,
    Asterisk,
    Percent,
    Increment,
    Decrement,

    Loop,
    While,
    Continue,
    Break,

    Compare,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,

    PlusEquals,
    SubtractEquals,
    DivideEquals,
    MultiplyEquals,
    PercentEquals,

    Boolean(bool),
    Character(String),
    String(String),
    Integer(String),
    Float(String),
    Identifier(String),
}
impl Token {
    pub fn better_eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Self::Boolean(_), Self::Boolean(_))
            | (Self::String(_), Self::String(_))
            | (Self::Integer(_), Self::Integer(_))
            | (Self::Float(_), Self::Float(_))
            | (Self::Identifier(_), Self::Identifier(_))
            | (Self::Character(_), Self::Character(_)) => true,
            _ => self == other,
        }
    }
}

pub fn match_token(word: &String) -> Option<Token> {
    let token = match &word[..] {
        "func" => Token::Function,
        "if" => Token::If,
        "else" => Token::Else,
        "elseif" => Token::ElseIf,

        "mut" => Token::Mutable,
        "var" => Token::Variable,

        "true" => Token::Boolean(true),
        "false" => Token::Boolean(false),

        "pub" => Token::Pub,
        "import" => Token::Import,
        "use" => Token::Use,

        "unsafe" => Token::Unsafe,

        "enum" => Token::Enum,
        "struct" => Token::Struct,

        "return" => Token::Return,
        "result" => Token::Result,

        "loop" => Token::Loop,
        "while" => Token::While,
        "break" => Token::Break,
        "continue" => Token::Continue,

        "{" => Token::OpenBlock,
        "}" => Token::CloseBlock,
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
        "[" => Token::OpenBracket,
        "]" => Token::CloseBracket,

        "&" => Token::Ampersand,
        "_" => Token::Underscore,
        "!" => Token::ExclamationMark,

        "<<" => Token::LeftBitshift,
        ">>" => Token::RightBitshift,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Asterisk,
        "/" => Token::ForwardSlash,
        "%" => Token::Percent,
        "++" => Token::Increment,
        "--" => Token::Decrement,

        "->" => Token::Arrow,
        "=>" => Token::FatArrow,

        "." => Token::Dot,
        "," => Token::Comma,
        ";" => Token::SemiColon,
        ":" => Token::Colon,
        "::" => Token::DoubleColon,

        "+=" => Token::PlusEquals,
        "-=" => Token::SubtractEquals,
        "/=" => Token::DivideEquals,
        "*=" => Token::MultiplyEquals,
        "%=" => Token::PercentEquals,

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
                Function => "func",
                OpenBlock => "{",
                CloseBlock => "}",
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
                Result => "result",
                Dot => ".",
                Underscore => "_",
                Equals => "=",
                Compare => "==",
                Comma => ",",
                Mutable => "mut",
                Variable => "var",
                If => "if",
                ElseIf => "elseif",
                Else => "else",

                LeftBitshift => "<<",
                RightBitshift => ">>",
                Plus => "+",
                Minus => "-",
                Asterisk => "*",
                ForwardSlash => "/",
                Percent => "%",
                Loop => "loop",
                While => "while",
                Continue => "continue",
                LessThan => "<",
                LessThanOrEquals => "<=",
                GreaterThan => ">",
                GreaterThanOrEquals => ">=",
                NotEquals => "!=",
                PlusEquals => "+=",
                SubtractEquals => "-=",
                DivideEquals => "/=",
                MultiplyEquals => "*=",
                PercentEquals => "%=",
                Increment => "++",
                Decrement => "--",
                Arrow => "->",
                FatArrow => "=>",

                Boolean(_) => "bool",
                Character(_) => "'x'",
                String(_) => "\"string\"",
                Integer(_) => "0-9",
                Float(_) => "0.0",
                Identifier(_) => "Identifier",
            }
        )
    }
}

pub type TokenInfo = Located<Token>;
impl std::fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Token: {} : {}", self.raw, self.position)
    }
}
