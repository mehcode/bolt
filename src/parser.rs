use failure::{err_msg, Error};
use tokenizer::Tokenizer;
use token::{Operator, Token};
use ast::*;

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub fn parse(&mut self) -> Result<Box<Expression>, Error> {
        Ok(Box::new(self.parse_expression(0)?))
    }

    pub fn parse_primary_expression(&mut self) -> Result<Expression, Error> {
        match self.tokenizer.next()? {
            Some((location, Token::Integer { value, .. })) => {
                Ok(Expression {
                    node: ExpressionKind::Literal(Box::new(Literal {
                        node: LiteralKind::Integer(value),
                        location: location.clone(),
                    })),

                    location,
                })
            },

            _ => {
                // FIXME: Better error message
                Err(err_msg("error: unexpected end of stream"))
            }
        }
    }

    pub fn parse_binary_expression(
        &mut self,
        operand: Expression,
        power: u32,
    ) -> Result<Option<Expression>, Error> {
        let op = if let Some((_, Token::Operator(op))) = self.tokenizer.peek()? {
            op
        } else {
            // FIXME: Error message here
            unreachable!();
        };

        let (op_power, op_assoc) = match op {
            Operator::Star => (1900, 1),
            Operator::Slash => (1900, 1),
            Operator::Plus => (1800, 1),
            Operator::Minus => (1800, 1),

            // FIXME: Error message here
            _ => unreachable!(),
        };

        if op_power < power {
            return Ok(None);
        }

        self.tokenizer.next()?;

        let operand_right = self.parse_expression(op_power + op_assoc)?;

        let operation = match op {
            Operator::Plus => BinaryOperation::Add,
            Operator::Minus => BinaryOperation::Subtract,
            Operator::Star => BinaryOperation::Multiply,
            Operator::Slash => BinaryOperation::Divide,

            // FIXME: Error message here
            _ => unreachable!(),
        };

        Ok(Some(Expression {
            location: operand.location.clone(),
            node: ExpressionKind::Binary(operation, Box::new(operand), Box::new(operand_right)),
        }))
    }

    pub fn parse_expression(&mut self, power: u32) -> Result<Expression, Error> {
        let mut expr = None;

        loop {
            let (_, tok) = match self.tokenizer.peek()? {
                Some(tok) => tok,
                None => break,
            };

            if let Some(operand) = expr.clone().take() {
                if tok.is_binary_operator() {
                    if let Some(result) = self.parse_binary_expression(operand, power)? {
                        expr = Some(result);

                        continue;
                    } else {
                        // Precedence rules say that we should not attempt to combine with
                        // the next operator
                        break;
                    }
                } else {
                    // Not a binary operator; we're done
                    break;
                }
            }

            expr = Some(self.parse_primary_expression()?);
        }

        if let Some(expr) = expr {
            Ok(expr)
        } else {
            // FIXME: Proper error message
            Err(err_msg("expected expression; found ?"))
        }
    }
}
