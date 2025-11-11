use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TokenKind {
    Eof,

    True,
    False,
    Number,
    String,
    Identifier,

    // Grouping & Braces
    Pipe,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,

    // Equivilance
    Equals,
    NotEquals,
    Not,
    Assignment,

    // Conditional
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    // Logical
    Or,
    And,

    // Symbols
    DotDot,
    Dot,
    SemiColon,
    DoubleColon,
    Colon,
    Question,
    Comma,

    // Shorthand
    PlusPlus,
    MinusMinus,
    PlusEquals,
    MinusEquals,

    DivideEquals,
    MultiplyEquals,
    ModEquals,

    //Maths
    Plus,
    Minus,
    Slash,
    Star,
    Percent,

    // Reserved Keywords
    Let,
    Mut,
    Const,
    Struct,
    Import,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Match,
    Pub,
    Return,
    Continue,
    Break,
}

impl TokenKind {
    #[must_use]
    pub fn is_keyword(input: &str) -> Option<TokenKind> {
        match input {
            "let" => Some(TokenKind::Let),
            "mut" => Some(TokenKind::Mut),
            "const" => Some(TokenKind::Const),
            "struct" => Some(TokenKind::Struct),
            "import" => Some(TokenKind::Import),
            "fn" => Some(TokenKind::Fn),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "while" => Some(TokenKind::While),
            "for" => Some(TokenKind::For),
            "in" => Some(TokenKind::In),
            "match" => Some(TokenKind::Match),
            "pub" => Some(TokenKind::Pub),
            "return" => Some(TokenKind::Return),
            "continue" => Some(TokenKind::Continue),
            "break" => Some(TokenKind::Break),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_binding_power(&self) -> u8 {
        use TokenKind::{
            And, Assignment, DotDot, Equals, Greater, GreaterEquals, Less, LessEquals, Minus,
            NotEquals, OpenParen, Or, Percent, Plus, Slash, Star,
        };
        match self {
            Assignment => 2,
            And | Or | DotDot => 3,
            Less | LessEquals | Greater | GreaterEquals | Equals | NotEquals => 4,
            Plus | Minus => 5,
            Slash | Star | Percent => 6,
            OpenParen => 8,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub pos: usize,
    pub binding_power: u8,
}

impl Token {
    #[must_use]
    pub fn new(kind: &TokenKind, value: String, line: usize, pos: usize) -> Token {
        Token {
            kind: kind.clone(),
            value,
            line,
            pos,
            binding_power: kind.get_binding_power(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::Identifier | TokenKind::String | TokenKind::Number => {
                write!(f, "{:?}({})", self.kind, self.value)
            }
            _ => write!(f, "{:?}", self.kind),
        }
    }
}
