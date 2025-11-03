use super::parser::*;
use super::token::*;
use crate::discord_script::ast::*;

impl Parser {
    pub(super) fn parse_unary_expression(&mut self) -> Result<Box<dyn Expression>, ParserErrors> {
        let operator = self.next_token()?;
        use super::token::TokenKind::*;
        match operator.kind {
            Minus | Not => {}
            _ => return Err(ParserErrors::UnexpectedTokenKind(operator)),
        }
        let expression = self.parse_expression(BindingPower::None)?;

        Ok(Box::new(UnaryExpression {
            operator,
            expression,
        }))
    }

    pub(super) fn parse_groupping_expression(
        &mut self,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        self.expect_token(TokenKind::OpenParen)?;
        let inner = self.parse_expression(BindingPower::None)?;
        self.expect_token(TokenKind::CloseParen)?;

        Ok(Box::new(GrouppingExpression { inner }))
    }

    pub(super) fn parse_binary_expression(
        &mut self,
        left: Box<dyn Expression>,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let op_token = self.next_token()?;
        let right = self.parse_expression(power)?;

        Ok(Box::new(BinaryExpression {
            left,
            operator: op_token.clone(),
            right,
        }))
    }

    pub(super) fn parse_primary_expression(&mut self) -> Result<Box<dyn Expression>, ParserErrors> {
        use super::token::TokenKind::*;
        let next_token = self.next_token()?;
        match next_token.kind {
            Number => {
                if let Ok(number) = next_token.value.parse::<i64>() {
                    return Ok(Box::new(IntegerExpression {
                        value: i128::from(number),
                    }));
                }
                if let Ok(number) = next_token.value.parse::<u64>() {
                    return Ok(Box::new(IntegerExpression {
                        value: i128::from(number),
                    }));
                }
                if let Ok(number) = next_token.value.parse::<f64>() {
                    return Ok(Box::new(FloatExpression { value: number }));
                }
                return Err(ParserErrors::NumberIsNotANumber(next_token));
            }
            String => Ok(Box::new(StringExpression {
                value: next_token.value,
            })),
            Identifier => Ok(Box::new(SymbolExpression {
                value: next_token.value,
            })),
            _ => {
                panic!("{next_token:#?} \n Should never happen")
            }
        }
    }

    pub(super) fn parse_assignment_expression(
        &mut self,
        left: Box<dyn Expression>,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let operator = self.next_token()?;
        let value = self.parse_expression(power)?;

        Ok(Box::new(AssignmentExpression {
            assigne: left,
            operator,
            value,
        }))
    }

    pub(super) fn parse_function_call(
        &mut self,
        left: Box<dyn Expression>,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let mut params = vec![];

        self.expect_token(TokenKind::OpenParen)?;

        let mut has_args = false;
        if let Err(e) = self.expect_token(TokenKind::CloseParen) {
            match e {
                ParserErrors::UnexpectedTokenKind(_) => has_args = true,
                _ => return Err(e),
            }
        }

        if has_args {
            params.push(self.parse_expression(power.clone())?);
            while let Ok(_) = self.expect_token(TokenKind::Comma) {
                params.push(self.parse_expression(BindingPower::None)?);
            }
        }

        self.expect_token(TokenKind::CloseParen)?;
        Ok(Box::new(FunctionCallExpression {
            params,
            identifier: left,
        }))
    }
}
