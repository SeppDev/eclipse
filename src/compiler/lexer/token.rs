use crate::{
    common::position::PositionRange,
    compiler::common::operators::{ArithmethicOperator, CompareOperator},
};

pub const MAX_OPERATOR_WIDTH: usize = 3;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    StartOfFile, // <sof>
    EndOfFile,   // <eof>
    Unkown,      // <unkown>

    OpenCurlyBracket,  // {
    CloseCurlyBracket, // }
    CloseParen,        // )
    CloseBracket,      // ]
    OpenBracket,       // [
    OpenParen,         // (

    Pub,    // pub
    Extern, // extern
    Async,  // async
    Unsafe, // unsafe
    Static, // Static

    Import,  // import
    Use,     // use
    Mutable, // mut
    Var,     // var
    Enum,    // enum
    Struct,  // struct

    Return, // return
    Result, // result

    Ampersand,       // &
    CommercialAt,    // @
    NumberSign,      // #
    SemiColon,       // ;
    DoubleColon,     // ::
    Dot,             // .
    Underscore,      // _
    Colon,           // :
    Equals,          // =
    Comma,           // ,
    ExclamationMark, // !
    Arrow,           // ->
    FatArrow,        // =>

    SelfKeyword, // self
    Super,       // super

    If,     // if
    ElseIf, // else if
    Else,   // else

    LeftBitshift,  // <<
    RightBitshift, // >>
    Plus,          // +
    Minus,         // -
    ForwardSlash,  // /
    Asterisk,      // *
    Percent,       // %
    Increment,     // ++
    Decrement,     // --
    Apostrophe,    // '

    Loop,     // loop
    While,    // while
    Continue, // continue
    Break,    // break
    Function, // func

    Range,               // ..
    RangeEquals,         // ..=
    Compare,             // ==
    NotEquals,           // !=
    LessThan,            // <
    LessThanOrEquals,    // <=
    GreaterThan,         // >
    GreaterThanOrEquals, // >=
    And,                 // &&
    Or,                  // ||

    PlusEquals,      // +=
    SubtractEquals,  // -=
    DivideEquals,    // /=
    MultiplyEquals,  // *=
    RemainderEquals, // %=

    Boolean,    // boolean
    Character,  // char
    String,     // string
    Integer,    // int
    Float,      // float
    Identifier, // identifier
}
impl TokenKind {
    pub fn is_expression(&self) -> bool {
        use TokenKind::*;

        match self {
            Identifier | Float | Integer | String | Character | Boolean | SelfKeyword => true,
            _ => false,
        }
    }
    pub fn is_expression_start(&self) -> bool {
        use TokenKind::*;

        match self {
            OpenParen | Minus | If | While | Loop => true,
            _ if self.is_expression() => true,
            _ => false,
        }
    }
    pub fn is_keyword(&self) -> bool {
        use TokenKind::*;

        match self {
            Loop | Continue | Break | While | If | ElseIf | Else | Pub | Use | Enum | Struct
            | Function | Unsafe | Return | Result | Var | SelfKeyword => true,
            _ => false,
        }
    }
    pub fn is_equals_operation(&self) -> bool {
        use TokenKind::*;

        match self {
            Equals | PlusEquals | RangeEquals | DivideEquals | RemainderEquals | MultiplyEquals
            | SubtractEquals => true,
            _ => false,
        }
    }
    pub fn is_arithmetic_operator(&self) -> bool {
        ArithmethicOperator::try_from(self).is_ok()
    }
    pub fn is_compare_operator(&self) -> bool {
        CompareOperator::try_from(self).is_ok()
    }
    pub fn is_operator(&self) -> bool {
        self.is_compare_operator() | self.is_arithmetic_operator()
    }
    pub fn is_modifier(&self) -> bool {
        use TokenKind::*;
        match self {
            Unsafe | Pub | Extern => true,
            _ => false,
        }
    }
}

pub fn match_token(word: &String) -> Option<TokenKind> {
    use TokenKind::*;
    let token = match &word[..] {
        "if" => If,
        "else" => Else,
        "elseif" => ElseIf,

        "mut" => Mutable,
        "var" => Var,

        "true" | "false" => Boolean,

        "super" => Super,

        "pub" => Pub,
        "static" => Static,
        "async" => Async,
        "unsafe" => Unsafe,
        "extern" => Extern,

        "enum" => Enum,
        "struct" => Struct,

        "func" => Function,
        "import" => Import,
        "use" => Use,
        "return" => Return,
        "result" => Result,
        "loop" => Loop,
        "while" => While,
        "break" => Break,
        "continue" => Continue,
        "self" => SelfKeyword,

        "{" => OpenCurlyBracket,
        "}" => CloseCurlyBracket,
        "(" => OpenParen,
        ")" => CloseParen,
        "[" => OpenBracket,
        "]" => CloseBracket,

        "&" => Ampersand,
        "_" => Underscore,
        "!" => ExclamationMark,
        "'" => Apostrophe,
        "@" => CommercialAt,
        "#" => NumberSign,
        ".." => Range,
        "..=" => RangeEquals,
        "<<" => LeftBitshift,
        ">>" => RightBitshift,
        "+" => Plus,
        "-" => Minus,
        "*" => Asterisk,
        "/" => ForwardSlash,
        "%" => Percent,
        "++" => Increment,
        "--" => Decrement,
        "&&" => And,
        "||" => Or,
        "->" => Arrow,
        "=>" => FatArrow,

        "." => Dot,
        "," => Comma,
        ";" => SemiColon,
        ":" => Colon,
        "::" => DoubleColon,

        "+=" => PlusEquals,
        "-=" => SubtractEquals,
        "/=" => DivideEquals,
        "*=" => MultiplyEquals,
        "%=" => RemainderEquals,

        "<" => LessThan,
        ">" => GreaterThan,
        "<=" => LessThanOrEquals,
        ">=" => GreaterThanOrEquals,
        "!=" => NotEquals,
        "==" => Compare,
        "=" => Equals,
        _ => return None,
    };

    return Some(token);
}

impl std::fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Token: {:?}({:?}) : {}",
            self.kind, self.string, self.position
        )
    }
}
