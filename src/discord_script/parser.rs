use std::collections::{HashMap, VecDeque};

use crate::discord_script::ast::*;

use super::ast::{Expression, Statement};
use super::token::{Token, TokenKind};

#[derive(Debug, PartialEq, Hash, PartialOrd, Clone)]
#[repr(i32)]
pub enum BindingPower {
    None,
    Comma,
    Assignment,
    Logical,
    Comparison,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary,
}

#[derive(Debug)]
pub enum ParserErrors {
    NoFunctionHandler(Token),
    UnexpectedExpressionType(Token),
    NextTokenNotFound,
    NumberIsNotANumber(Token),
    BindingPowerError,
    UnexpectedTokenKind(Token),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum LeftDenotationHandlerTypes {
    Default,
    Assignment,
    FunctionCall,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum NullDenotationHandlerTypes {
    Default,
    Groupping,
    Unary,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum StatementHandlerTypes {
    Default,
    Variable,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    left_denotation_lookup: HashMap<TokenKind, LeftDenotationHandlerTypes>,
    null_denotation_lookup: HashMap<TokenKind, NullDenotationHandlerTypes>,
    statement_lookup: HashMap<TokenKind, StatementHandlerTypes>,
    binding_power_lookup: HashMap<TokenKind, BindingPower>,
}

impl Parser {
    fn try_parse_null_denotaion(
        &mut self,
        kind: TokenKind,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let handler_type =
            self.null_denotation_lookup
                .get(&kind)
                .ok_or(ParserErrors::NoFunctionHandler(
                    self.current_token().clone(),
                ))?;

        return match handler_type {
            NullDenotationHandlerTypes::Default => self.parse_primary_expression(),
            NullDenotationHandlerTypes::Groupping => self.parse_groupping_expression(),
            NullDenotationHandlerTypes::Unary => self.parse_unary_expression(),
        };
    }

    fn try_parse_left_denotation(
        &mut self,
        left: Box<dyn Expression>,
        new_power: BindingPower,
        kind: TokenKind,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let function_type = self.left_denotation_lookup.get(&kind).ok_or(
            ParserErrors::UnexpectedExpressionType(self.current_token().clone()),
        )?;

        return match function_type {
            LeftDenotationHandlerTypes::Default => {
                self.parse_binary_expression(left, new_power.clone())
            }
            LeftDenotationHandlerTypes::Assignment => {
                self.parse_assignment_expression(left, new_power.clone())
            }
            LeftDenotationHandlerTypes::FunctionCall => {
                self.parse_function_call(left, new_power.clone())
            }
        };
    }

    pub(super) fn parse_expression(
        &mut self,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let mut kind = self.current_token().kind.clone();

        let mut left = self.try_parse_null_denotaion(kind)?;

        while *self
            .binding_power_lookup
            .get(&self.current_token().kind)
            .unwrap_or(&BindingPower::None)
            > power
        {
            kind = self.current_token().kind.clone();

            let new_power = self
                .binding_power_lookup
                .get(&self.current_token().kind)
                .ok_or(ParserErrors::BindingPowerError)?;

            left = self.try_parse_left_denotation(left, new_power.clone(), kind)?;
        }

        Ok(left)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParserErrors> {
        let statement_handler_type = self.statement_lookup.get(&self.current_token().kind);
        if let Some(statement_handler_type) = statement_handler_type {
            match *statement_handler_type {
                StatementHandlerTypes::Default => return self.parse_statement(),
                StatementHandlerTypes::Variable => return self.parse_variable_statement(),
            };
        }

        let expression = self.parse_expression(BindingPower::None)?;
        self.expect_token(TokenKind::SemiColon)?;

        return Ok(Box::new(ExpressionStatement { expression }));
    }

    fn has_tokens(&self) -> bool {
        self.current_token().kind != TokenKind::Eof
    }

    pub fn parse(tokens: Vec<Token>) -> Result<impl Statement, ParserErrors> {
        let mut body: Vec<Box<dyn Statement>> = Vec::new();
        let mut parser = Parser::new(tokens);

        while parser.has_tokens() {
            body.push(parser.parse_statement()?)
        }

        Ok(BlockStatement { body })
    }

    pub(super) fn current_token(&self) -> &Token {
        return &self.tokens.front().unwrap();
    }

    pub(super) fn next_token(&mut self) -> Result<Token, ParserErrors> {
        self.tokens
            .pop_front()
            .ok_or(ParserErrors::NextTokenNotFound)
    }

    fn new(tokens: Vec<Token>) -> Self {
        let mut res = Self {
            tokens: tokens.into(),
            left_denotation_lookup: HashMap::new(),
            null_denotation_lookup: HashMap::new(),
            statement_lookup: HashMap::new(),
            binding_power_lookup: HashMap::new(),
        };
        res.create_lookup_table();

        res
    }

    pub(super) fn expect_token(&mut self, kind: TokenKind) -> Result<Token, ParserErrors> {
        if kind != self.current_token().kind {
            return Err(ParserErrors::UnexpectedTokenKind(
                self.current_token().clone(),
            ));
        }
        let token = self.next_token()?;

        return Ok(token);
    }

    pub(self) fn create_lookup_table(&mut self) {
        use super::token::TokenKind::*;
        use BindingPower::*;

        let mut add_new =
            |kind: TokenKind, power: BindingPower, h_type: LeftDenotationHandlerTypes| {
                self.binding_power_lookup.insert(kind.clone(), power);
                self.left_denotation_lookup.insert(kind, h_type);
            };

        self.null_denotation_lookup
            .insert(Number, NullDenotationHandlerTypes::Default);
        self.null_denotation_lookup
            .insert(String, NullDenotationHandlerTypes::Default);
        self.null_denotation_lookup
            .insert(Identifier, NullDenotationHandlerTypes::Default);

        self.null_denotation_lookup
            .insert(Minus, NullDenotationHandlerTypes::Unary);
        self.null_denotation_lookup
            .insert(Not, NullDenotationHandlerTypes::Unary);
        self.null_denotation_lookup
            .insert(OpenParen, NullDenotationHandlerTypes::Groupping);
        //Logical
        add_new(And, Logical, LeftDenotationHandlerTypes::Default);
        add_new(Or, Logical, LeftDenotationHandlerTypes::Default);
        add_new(DotDot, Logical, LeftDenotationHandlerTypes::Default);

        //Comparison
        add_new(Less, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(LessEquals, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(Greater, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(
            GreaterEquals,
            Comparison,
            LeftDenotationHandlerTypes::Default,
        );
        add_new(Equals, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(NotEquals, Comparison, LeftDenotationHandlerTypes::Default);

        //Math
        add_new(Plus, Additive, LeftDenotationHandlerTypes::Default);
        add_new(Minus, Additive, LeftDenotationHandlerTypes::Default);

        add_new(Star, Multiplicative, LeftDenotationHandlerTypes::Default);
        add_new(Divide, Multiplicative, LeftDenotationHandlerTypes::Default);
        add_new(Percent, Multiplicative, LeftDenotationHandlerTypes::Default);

        add_new(OpenParen, Call, LeftDenotationHandlerTypes::FunctionCall);

        //
        add_new(
            TokenKind::Assignment,
            BindingPower::Assignment,
            LeftDenotationHandlerTypes::Assignment,
        );

        //Statements
        self.statement_lookup
            .insert(Let, StatementHandlerTypes::Variable);
        self.statement_lookup
            .insert(Const, StatementHandlerTypes::Variable);
    }
}
