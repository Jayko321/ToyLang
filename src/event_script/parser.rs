use std::collections::{HashMap, VecDeque};

use crate::event_script::{
    ast::{Expression, Statement},
    token::{Token, TokenKind},
};

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
    // FunctionCall,
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
    Block,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    left_denotation_lookup: HashMap<TokenKind, LeftDenotationHandlerTypes>,
    null_denotation_lookup: HashMap<TokenKind, NullDenotationHandlerTypes>,
    statement_lookup: HashMap<TokenKind, StatementHandlerTypes>,
}

impl Parser {
    pub(super) fn new(tokens: VecDeque<Token>) -> Self {
        let mut res = Self {
            tokens,
            left_denotation_lookup: HashMap::new(),
            null_denotation_lookup: HashMap::new(),
            statement_lookup: HashMap::new(),
        };

        res.initialize();
        res
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, ParserErrors> {
        let mut parser = Parser::new(tokens.into());
        let mut body: Vec<Statement> = Vec::new();
        while parser.has_tokens() {
            body.push(parser.parse_statement()?);
        }

        Ok(body)
    }

    fn has_tokens(&self) -> bool {
        self.current_token().kind != TokenKind::Eof
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn try_parse_null_denotaion(&mut self, kind: &TokenKind) -> Result<Expression, ParserErrors> {
        let handler_type =
            self.null_denotation_lookup
                .get(kind)
                .ok_or(ParserErrors::NoFunctionHandler(
                    self.current_token().clone(),
                ))?;

        match handler_type {
            NullDenotationHandlerTypes::Default => self.parse_primary_expression(),
            NullDenotationHandlerTypes::Groupping => self.parse_groupping_expression(),
            NullDenotationHandlerTypes::Unary => self.parse_unary_expression(),
        }
    }

    fn try_parse_left_denotation(
        &mut self,
        left: Expression,
        new_power: u8,
        kind: &TokenKind,
    ) -> Result<Expression, ParserErrors> {
        let function_type =
            self.left_denotation_lookup
                .get(kind)
                .ok_or(ParserErrors::UnexpectedExpressionType(
                    self.current_token().clone(),
                ))?;

        match function_type {
            LeftDenotationHandlerTypes::Default => self.parse_binary_expression(left, new_power),
            LeftDenotationHandlerTypes::Assignment => self.parse_assignment_expression(new_power), // LeftDenotationHandlerTypes::FunctionCall => {
                                                                                                   //     self.parse_function_call(left, new_power.clone())
                                                                                                   // }
        }
    }

    pub(super) fn parse_expression(
        &mut self,
        binding_power: u8,
    ) -> Result<Expression, ParserErrors> {
        let mut kind = self.current_token().kind.clone();

        let mut left = self.try_parse_null_denotaion(&kind)?;

        while self.current_token().binding_power > binding_power {
            kind = self.current_token().kind.clone();

            let new_power = self.current_token().binding_power;
            left = self.try_parse_left_denotation(left, new_power, &kind)?;
        }

        Ok(left)
    }

    pub(super) fn parse_statement(&mut self) -> Result<Statement, ParserErrors> {
        let statement_handler_type = self.statement_lookup.get(&self.current_token().kind);
        if let Some(statement_handler_type) = statement_handler_type {
            match *statement_handler_type {
                StatementHandlerTypes::Default => return self.parse_statement(),
                StatementHandlerTypes::Variable => return self.parse_variable_statement(),
                StatementHandlerTypes::Block => return self.parse_block_statement(),
            };
        }

        let expression = self.parse_expression(0)?;
        self.expect_token(&TokenKind::SemiColon)?;

        Ok(Statement::Expression(expression))
    }

    pub(super) fn expect_any_token(
        &mut self,
        kinds: &Vec<&TokenKind>,
    ) -> Result<Token, ParserErrors> {
        let is_correct = kinds.iter().any(|val| **val == self.current_token().kind);

        if is_correct {
            Ok(self.next_token()?)
        } else {
            Err(ParserErrors::UnexpectedTokenKind(
                self.current_token().clone(),
            ))
        }
    }

    pub(super) fn expect_token(&mut self, kind: &TokenKind) -> Result<Token, ParserErrors> {
        if *kind != self.current_token().kind {
            return Err(ParserErrors::UnexpectedTokenKind(
                self.current_token().clone(),
            ));
        }
        let token = self.next_token()?;

        Ok(token)
    }

    pub(super) fn current_token(&self) -> &Token {
        self.tokens.front().unwrap()
    }

    pub(super) fn next_token(&mut self) -> Result<Token, ParserErrors> {
        self.tokens
            .pop_front()
            .ok_or(ParserErrors::NextTokenNotFound)
    }

    fn initialize(&mut self) {
        use super::token::TokenKind::{
            And, Const, DotDot, Equals, Greater, GreaterEquals, Identifier, Less, LessEquals, Let,
            Minus, Not, NotEquals, Number, OpenCurly, OpenParen, Or, Percent, Plus, Slash, Star,
            String,
        };

        let mut add_new = |kind: TokenKind, h_type: LeftDenotationHandlerTypes| {
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
        add_new(And, LeftDenotationHandlerTypes::Default);
        add_new(Or, LeftDenotationHandlerTypes::Default);
        add_new(DotDot, LeftDenotationHandlerTypes::Default);

        //Comparison
        add_new(Less, LeftDenotationHandlerTypes::Default);
        add_new(LessEquals, LeftDenotationHandlerTypes::Default);
        add_new(Greater, LeftDenotationHandlerTypes::Default);
        add_new(GreaterEquals, LeftDenotationHandlerTypes::Default);
        add_new(Equals, LeftDenotationHandlerTypes::Default);
        add_new(NotEquals, LeftDenotationHandlerTypes::Default);

        //Math
        add_new(Plus, LeftDenotationHandlerTypes::Default);
        add_new(Minus, LeftDenotationHandlerTypes::Default);

        add_new(Star, LeftDenotationHandlerTypes::Default);
        add_new(Slash, LeftDenotationHandlerTypes::Default);
        add_new(Percent, LeftDenotationHandlerTypes::Default);

        // add_new(OpenParen, LeftDenotationHandlerTypes::FunctionCall);

        //
        add_new(
            TokenKind::Assignment,
            LeftDenotationHandlerTypes::Assignment,
        );

        //Statements
        self.statement_lookup
            .insert(Let, StatementHandlerTypes::Variable);
        self.statement_lookup
            .insert(Const, StatementHandlerTypes::Variable);
        self.statement_lookup
            .insert(OpenCurly, StatementHandlerTypes::Block);
    }
}
