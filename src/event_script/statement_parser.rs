use super::parser::*;
use super::token::*;
use crate::event_script::ast::*;

impl Parser {
    pub(super) fn parse_block_statement(&mut self) -> Result<Statement, ParserErrors> {
        self.expect_token(TokenKind::OpenCurly)?;
        let mut statements = Vec::new();
        while self.current_token().kind != TokenKind::CloseCurly {
            statements.push(self.parse_statement()?);
        }

        self.expect_token(TokenKind::CloseCurly)?;

        Ok(Statement::Block(statements))
    }

    pub(super) fn parse_variable_statement(&mut self) -> Result<Statement, ParserErrors> {
        let let_token = self.next_token()?;
        let is_const = match let_token.kind {
            TokenKind::Const => true,
            TokenKind::Let => false,
            _ => {
                return Err(ParserErrors::UnexpectedTokenKind(let_token));
            }
        };

        let name_token = self.expect_token(TokenKind::Identifier)?;
        //let after_name = self.next_token()?;
        let has_explicit_type = match self.current_token().kind {
            TokenKind::Colon => true,
            TokenKind::Assignment => false,
            _ => {
                return Err(ParserErrors::UnexpectedTokenKind(
                    self.current_token().clone(),
                ));
            }
        };
        let mut explicit_type_val = None;
        if has_explicit_type {
            self.expect_token(TokenKind::Colon)?;
            let explicit_type = self.next_token()?;
            if explicit_type.kind != TokenKind::Identifier {
                return Err(ParserErrors::UnexpectedTokenKind(explicit_type));
            }
            explicit_type_val = Some(explicit_type.value);
            //self.expect_token(TokenKind::Assignment)?;
        }

        let token = self.expect_any_token(vec![TokenKind::Assignment, TokenKind::SemiColon])?;
        let mut expr = None;
        match token.kind {
            TokenKind::Assignment => {
                expr = Some(self.parse_expression(0)?);
                self.expect_token(TokenKind::SemiColon)?;
            }
            TokenKind::SemiColon => {}
            _ => {
                return Err(ParserErrors::UnexpectedTokenKind(token));
            }
        }

        Ok(Statement::Variable(
            name_token.value,
            is_const,
            explicit_type_val,
            expr,
        ))
    }
}
