use crate::event_script::token::{Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    String(String),
    Number(String),
    Float(String),
    Groupping(Box<Expression>),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, TokenKind, Box<Expression>),
    Symbol(String),
    Assignment(Box<Expression>),
    // FunctionCall,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
    Block(Vec<Statement>),
    Variable(String, bool, bool, Option<String>, Option<Expression>),
}
