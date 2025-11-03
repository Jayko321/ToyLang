use super::parser::*;
use super::token::*;
use crate::discord_script::ast::*;

impl Parser {
    pub(super) fn parse_variable_statement(&mut self) -> Result<Box<dyn Statement>, ParserErrors> {
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

        self.expect_token(TokenKind::Assignment)?;

        let expr = self.parse_expression(BindingPower::None)?;

        self.expect_token(TokenKind::SemiColon)?;

        return Ok(Box::new(VariableStatement {
            is_const,
            variable_name: name_token.value,
            explicit_type: explicit_type_val,
            assignment: expr,
        }));
    }
}
