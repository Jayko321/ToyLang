use super::parser::*;
use super::token::*;
use crate::event_script::ast::*;

impl Parser {
    pub(super) fn parse_unary_expression(&mut self) -> Result<Expression, ParserErrors> {
        let operator = self.next_token()?;
        use super::token::TokenKind::*;
        match operator.kind {
            Minus | Not => {}
            _ => return Err(ParserErrors::UnexpectedTokenKind(operator)),
        }
        let expression = self.parse_expression(0)?;

        Ok(Expression::Unary(operator, Box::new(expression)))
    }

    pub(super) fn parse_groupping_expression(&mut self) -> Result<Expression, ParserErrors> {
        self.expect_token(TokenKind::OpenParen)?;
        let inner = self.parse_expression(0)?;
        self.expect_token(TokenKind::CloseParen)?;

        Ok(Expression::Groupping(Box::new(inner)))
    }

    pub(super) fn parse_binary_expression(
        &mut self,
        left: Expression,
        power: u8,
    ) -> Result<Expression, ParserErrors> {
        let op_token = self.next_token()?;
        let right = self.parse_expression(power)?;

        Ok(Expression::Binary(
            Box::new(left),
            op_token.kind.clone(),
            Box::new(right),
        ))
    }

    pub(super) fn parse_primary_expression(&mut self) -> Result<Expression, ParserErrors> {
        use super::token::TokenKind::*;
        let next_token = self.next_token()?;
        match next_token.kind {
            Number => Ok(Expression::Number(next_token.value)),
            String => Ok(Expression::String(next_token.value)),
            Identifier => Ok(Expression::Symbol(next_token.value)),
            _ => {
                panic!("{next_token:#?} \n Should never happen")
            }
        }
    }

    pub(super) fn parse_assignment_expression(
        &mut self,
        power: u8,
    ) -> Result<Expression, ParserErrors> {
        self.next_token()?;
        let value = self.parse_expression(power)?;

        Ok(Expression::Assignment(Box::new(value)))
    }

    // pub(super) fn parse_function_call(
    //     &mut self,
    //     left: Expression,
    //     power: u8,
    // ) -> Result<Expression, ParserErrors> {
    //     let mut params = vec![];
    //
    //     self.expect_token(TokenKind::OpenParen)?;
    //
    //     let mut has_args = false;
    //     if let Err(e) = self.expect_token(TokenKind::CloseParen) {
    //         match e {
    //             ParserErrors::UnexpectedTokenKind(_) => has_args = true,
    //             _ => return Err(e),
    //         }
    //     }
    //
    //     if has_args {
    //         params.push(self.parse_expression(power.clone())?);
    //         while let Ok(_) = self.expect_token(TokenKind::Comma) {
    //             params.push(self.parse_expression(0)?);
    //         }
    //     }
    //
    //     self.expect_token(TokenKind::CloseParen)?;
    //     Ok(Box::new(FunctionCallExpression {
    //         params,
    //         identifier: left,
    //     }))
    // }
}
