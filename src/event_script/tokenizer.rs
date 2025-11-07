use std::fmt::Display;
use std::ops::Deref;

use crate::event_script::token::*;
use lazy_regex::{Regex, regex};

pub struct Lexer {
    tokens: Vec<Token>,
    input: String,
    pos: usize,
    patterns: Vec<RegexPattern>,
}

pub type HandlerType = dyn Fn(&Regex, &str, usize, usize) -> (usize, Option<Token>);

pub struct RegexPattern {
    regex: regex::Regex,
    handler: Box<HandlerType>,
}

impl RegexPattern {
    pub fn new(regex: Regex, handler: Box<HandlerType>) -> Self {
        RegexPattern { regex, handler }
    }
}

#[derive(Debug)]
pub enum TokenizerError {
    InvalidToken(String),
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TokenizerError {}

fn default_handler(kind: TokenKind, value: &'static str) -> Box<HandlerType> {
    let res = move |_: &Regex, _: &str, line: usize, pos: usize| -> (usize, Option<Token>) {
        (
            value.len(),
            Some(Token::new(kind.clone(), value.to_string(), line, pos)),
        )
    };
    Box::new(res)
}

fn skip_handler(regex: &Regex, remainder: &str, _: usize, _: usize) -> (usize, Option<Token>) {
    if let Some(pat) = regex.find(remainder) {
        return (pat.end(), None);
    }
    (0, None)
}

fn string_handler(
    regex: &Regex,
    remainder: &str,
    line: usize,
    pos: usize,
) -> (usize, Option<Token>) {
    if let Some(pat) = regex.find(remainder) {
        let str_value = pat.as_str().trim_matches('\"');
        return (
            str_value.len() + 2,
            Some(Token::new(
                TokenKind::String,
                str_value.to_string(),
                line,
                pos,
            )),
        );
    }
    (0, None)
}

fn number_handler(
    regex: &Regex,
    remainder: &str,
    line: usize,
    pos: usize,
) -> (usize, Option<Token>) {
    if let Some(pat) = regex.find(remainder) {
        let num_value = pat.as_str();
        return (
            num_value.len(),
            Some(Token::new(
                TokenKind::Number,
                String::from(num_value),
                line,
                pos,
            )),
        );
    }
    (0, None)
}

fn symbol_handler(
    regex: &Regex,
    remainder: &str,
    line: usize,
    pos: usize,
) -> (usize, Option<Token>) {
    if let Some(pat) = regex.find(remainder) {
        let value = pat.as_str();
        if let Some(keyword) = TokenKind::is_keyword(value) {
            return (
                value.len(),
                Some(Token::new(keyword, value.to_string(), line, pos)),
            );
        } else {
            return (
                value.len(),
                Some(Token::new(
                    TokenKind::Identifier,
                    String::from(value),
                    line,
                    pos,
                )),
            );
        }
    }
    (0, None)
}

impl Lexer {
    fn new(source: String) -> Self {
        use crate::event_script::token::TokenKind::*;

        #[rustfmt::skip]
        let patterns = vec![
            //Grouping
            RegexPattern::new(regex!(r#"\|"#).deref().to_owned(), default_handler(Pipe, "|")),
            RegexPattern::new(regex!(r#"\["#).deref().to_owned(), default_handler(OpenBracket, "[")),
            RegexPattern::new(regex!(r#"\]"#).deref().to_owned(), default_handler(CloseBracket, "]")),
            RegexPattern::new(regex!(r#"\{"#).deref().to_owned(), default_handler(OpenCurly, "{")),
            RegexPattern::new(regex!(r#"\}"#).deref().to_owned(), default_handler(CloseCurly, "}")),
            RegexPattern::new(regex!(r#"\("#).deref().to_owned(), default_handler(OpenParen, "(")),
            RegexPattern::new(regex!(r#"\)"#).deref().to_owned(), default_handler(CloseParen, ")")),
            //Equivilance
            RegexPattern::new(regex!(r#"=="#).deref().to_owned(), default_handler(Equals, "==")),
            RegexPattern::new(regex!(r#"!="#).deref().to_owned(), default_handler(NotEquals, "!=")),
            RegexPattern::new(regex!(r#"="#).deref().to_owned(), default_handler(Assignment, "=")),
            RegexPattern::new(regex!(r#"!"#).deref().to_owned(), default_handler(Not, "!")),
            //Conditional
            RegexPattern::new(regex!(r#"<="#).deref().to_owned(), default_handler(LessEquals, "<=")),
            RegexPattern::new(regex!(r#"<"#).deref().to_owned(), default_handler(Less, "<")),
            RegexPattern::new(regex!(r#">="#).deref().to_owned(), default_handler(GreaterEquals, ">=")),
            RegexPattern::new(regex!(r#">"#).deref().to_owned(), default_handler(Greater, ">")),
            //Logical
            RegexPattern::new(regex!(r#"\|\|"#).deref().to_owned(), default_handler(Or, "||")),
            RegexPattern::new(regex!(r#"&&"#).deref().to_owned(), default_handler(And, "&&")),
            //Symbols
            RegexPattern::new(regex!(r#"\.\."#).deref().to_owned(), default_handler(DotDot, "..")),
            RegexPattern::new(regex!(r#"\."#).deref().to_owned(), default_handler(Dot, ".")),
            RegexPattern::new(regex!(r#";"#).deref().to_owned(), default_handler(SemiColon, ";")),
            RegexPattern::new(regex!(r#"::"#).deref().to_owned(), default_handler(DoubleColon, "::")),
            RegexPattern::new(regex!(r#":"#).deref().to_owned(), default_handler(Colon, ":")),
            RegexPattern::new(regex!(r#"\?"#).deref().to_owned(), default_handler(Question, "?")),
            RegexPattern::new(regex!(r#","#).deref().to_owned(), default_handler(Comma, ",")),
            //Shorthand
            RegexPattern::new(regex!(r#"\+\+"#).deref().to_owned(), default_handler(PlusPlus, "++")),
            RegexPattern::new(regex!(r#"--"#).deref().to_owned(), default_handler(MinusMinus, "--")),
            RegexPattern::new(regex!(r#"\+="#).deref().to_owned(), default_handler(PlusEquals, "+=")),
            RegexPattern::new(regex!(r#"-="#).deref().to_owned(), default_handler(MinusEquals, "-=")),
            RegexPattern::new(regex!(r#"/="#).deref().to_owned(), default_handler(DivideEquals, "/=")),
            RegexPattern::new(regex!(r#"\*="#).deref().to_owned(), default_handler(MultiplyEquals, "*=")),
            RegexPattern::new(regex!(r#"\%="#).deref().to_owned(), default_handler(ModEquals, "%=")),
            //Maths
            RegexPattern::new(regex!(r#"\+"#).deref().to_owned(), default_handler(Plus, "+")),
            RegexPattern::new(regex!(r#"-"#).deref().to_owned(), default_handler(Minus, "-")),
            RegexPattern::new(regex!(r#"/"#).deref().to_owned(), default_handler(Slash, "/")),
            RegexPattern::new(regex!(r#"\*"#).deref().to_owned(), default_handler(Star, "*")),
            RegexPattern::new(regex!(r#"%"#).deref().to_owned(), default_handler(Percent, "%")),
            //Special cases
            RegexPattern::new(regex!(r#"\s+"#).deref().to_owned(), Box::new(skip_handler)),
            RegexPattern::new(regex!(r#"\/\/.*"#).deref().to_owned(), Box::new(skip_handler)),
            RegexPattern::new(regex!(r#""[^"]*""#).deref().to_owned(), Box::new(string_handler)),
            RegexPattern::new(regex!(r#"[0-9]+(\.[0-9]+)?"#).deref().to_owned(), Box::new(number_handler)),
            RegexPattern::new(regex!(r#"[a-zA-Z_][a-zA-Z0-9_!]*"#).deref().to_owned(), Box::new(symbol_handler)),
        ];
        Self {
            tokens: vec![],
            input: source,
            pos: 0,
            patterns,
        }
    }

    fn remainder(&self) -> String {
        self.input[self.pos..].to_string()
    }

    fn at_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn tokenize(source: String) -> Result<Vec<Token>, TokenizerError> {
        let mut lexer = Lexer::new(source);

        while !lexer.at_eof() {
            let mut matched = false;

            let remainder = lexer.remainder().to_owned();
            let mut advance = 0;
            let mut token = None;
            for pattern in &mut lexer.patterns {
                if let Some(location) = pattern.regex.find(&remainder)
                    && location.start() == 0
                {
                    (advance, token) = (pattern.handler)(&pattern.regex, &remainder, lexer.pos, 0);
                    matched = true;
                    break;
                }
            }

            if !matched {
                let pos = lexer.pos;
                return Err(TokenizerError::InvalidToken(format!(
                    "pos: {pos} source_len: {}",
                    lexer.input.len()
                )));
            }

            if let Some(token) = token {
                lexer.tokens.push(token);
            }
            lexer.pos += advance;
        }

        lexer
            .tokens
            .push(Token::new(TokenKind::Eof, "".to_string(), lexer.pos, 0));

        Ok(lexer.tokens)
    }
}
