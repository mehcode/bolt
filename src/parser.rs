use node::{Node, Expression, Literal, BinaryOperation};
use failure::{Error, err_msg};
use tokenizer::Tokenizer;
use token::{Token, Operator};

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub fn parse(&mut self) -> Result<Node, Error> {
        Ok(Node::Expression(self.parse_expression(0)?))
    }

    pub fn parse_primary_expression(&mut self) -> Result<Expression, Error> {
        match self.tokenizer.next()? {
            Some((_, Token::Integer { value, .. })) => {
                // FIXME: Base conversion
                let lit = Literal::Integer { value };

                Ok(Expression::Literal(lit))
            }

            _ => {
                // FIXME: Better error message
                Err(err_msg("error: unexpected end of stream"))
            }
        }
    }

    pub fn parse_binary_expression(&mut self, operand: Expression, power: u32) -> Result<Option<Expression>, Error> {
        let op = if let Some((_, Token::Operator(op))) = self.tokenizer.peek()? { op } else {
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
            return Ok(None)
        }

        self.tokenizer.next()?;

        let operand_right = self.parse_expression(op_power + op_assoc)?;

        Ok(Some(match op {
            Operator::Plus => {
                Expression::BinaryOperation(BinaryOperation::Add(
                    box Node::Expression(operand),
                    box Node::Expression(operand_right),
                ))
            },

            Operator::Minus => {
                Expression::BinaryOperation(BinaryOperation::Subtract(
                    box Node::Expression(operand),
                    box Node::Expression(operand_right),
                ))
            },

            Operator::Star => {
                Expression::BinaryOperation(BinaryOperation::Multiply(
                    box Node::Expression(operand),
                    box Node::Expression(operand_right),
                ))
            },

            Operator::Slash => {
                Expression::BinaryOperation(BinaryOperation::Divide(
                    box Node::Expression(operand),
                    box Node::Expression(operand_right),
                ))
            },

            // FIXME: Error message here
            _ => unreachable!()
        }))
    }

    pub fn parse_expression(&mut self, power: u32) -> Result<Expression, Error> {
        let mut expr = None;

        loop {
            let (_, tok) = match self.tokenizer.peek()? {
                Some(tok) => tok,
                None => break
            };

            if let Some(operand) = expr.clone().take() {
                if tok.is_binary_operator() {
                    if let Some(result) = self.parse_binary_expression(operand, power)? {
                        expr = Some(result);

                        continue
                    } else {
                        // Precedence rules say that we should not attempt to combine with
                        // the next operator
                        break
                    }
                } else  {
                    // Not a binary operator; we're done
                    break
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
