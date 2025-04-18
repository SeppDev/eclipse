use crate::common::position::PositionRange;

pub const MAX_OPERATOR_WIDTH: usize = 3;

#[derive(Debug)]
pub struct TokenInfo {
    pub kind: TokenKind,
    pub position: PositionRange,
    pub string: String,
}
impl TokenInfo {
    pub fn new(string: String, kind: TokenKind, position: PositionRange) -> Self {
        Self {
            string,
            kind,
            position,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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
    Var,
    ExclamationMark,
    Arrow,
    FatArrow,

    SelfType,
    SelfKeyword,

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

    Range,
    RangeEquals,
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

    Boolean,
    Character,
    String,
    Integer,
    Float,
    Identifier,
}

impl TokenKind {
    pub fn is_expression(&self) -> bool {
        use TokenKind::*;

        match self {
            Identifier | Float | Integer | String | Character | Boolean => true,
            _ => false,
        }
    }
    pub fn is_equals_operator(&self) -> bool {
        use TokenKind::*;

        match self {
            Equals | PlusEquals | RangeEquals | PercentEquals | MultiplyEquals | SubtractEquals => true,
            _ => false,
        }
    }
    pub fn is_arithmetic_operator(&self) -> bool {
        use TokenKind::*;

        match self {
            Plus | Minus | ForwardSlash | Asterisk | Percent => true,
            _ => false,
        }
    }
    pub fn is_compare_operator(&self) -> bool {
        use TokenKind::*;

        match self {
            Compare | GreaterThan | GreaterThanOrEquals | LessThan | LessThanOrEquals | NotEquals => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        return self.is_arithmetic_operator() | self.is_compare_operator();
    }
}

pub fn match_token(word: &String) -> Option<TokenKind> {
    let token = match &word[..] {
        "func" => TokenKind::Function,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "elseif" => TokenKind::ElseIf,

        "mut" => TokenKind::Mutable,
        "var" => TokenKind::Var,

        "true" | "false" => TokenKind::Boolean,

        "pub" => TokenKind::Pub,
        "import" => TokenKind::Import,
        "use" => TokenKind::Use,

        "unsafe" => TokenKind::Unsafe,

        "enum" => TokenKind::Enum,
        "struct" => TokenKind::Struct,

        "return" => TokenKind::Return,
        "result" => TokenKind::Result,

        "loop" => TokenKind::Loop,
        "while" => TokenKind::While,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,

        "Self" => TokenKind::SelfType,
        "self" => TokenKind::SelfKeyword,

        "{" => TokenKind::OpenBlock,
        "}" => TokenKind::CloseBlock,
        "(" => TokenKind::OpenParen,
        ")" => TokenKind::CloseParen,
        "[" => TokenKind::OpenBracket,
        "]" => TokenKind::CloseBracket,

        "&" => TokenKind::Ampersand,
        "_" => TokenKind::Underscore,
        "!" => TokenKind::ExclamationMark,

        ".." => TokenKind::Range,
        "..=" => TokenKind::RangeEquals,
        "<<" => TokenKind::LeftBitshift,
        ">>" => TokenKind::RightBitshift,
        "+" => TokenKind::Plus,
        "-" => TokenKind::Minus,
        "*" => TokenKind::Asterisk,
        "/" => TokenKind::ForwardSlash,
        "%" => TokenKind::Percent,
        "++" => TokenKind::Increment,
        "--" => TokenKind::Decrement,

        "->" => TokenKind::Arrow,
        "=>" => TokenKind::FatArrow,

        "." => TokenKind::Dot,
        "," => TokenKind::Comma,
        ";" => TokenKind::SemiColon,
        ":" => TokenKind::Colon,
        "::" => TokenKind::DoubleColon,

        "+=" => TokenKind::PlusEquals,
        "-=" => TokenKind::SubtractEquals,
        "/=" => TokenKind::DivideEquals,
        "*=" => TokenKind::MultiplyEquals,
        "%=" => TokenKind::PercentEquals,

        "<" => TokenKind::LessThan,
        ">" => TokenKind::GreaterThan,
        "<=" => TokenKind::LessThanOrEquals,
        ">=" => TokenKind::GreaterThanOrEquals,
        "!=" => TokenKind::NotEquals,
        "==" => TokenKind::Compare,
        "=" => TokenKind::Equals,
        _ => return None,
    };

    return Some(token);
}

impl std::fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Token: {:?} : {}", self.kind, self.position)
    }
}
