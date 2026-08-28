use crate::{
    ast::*,
    error::CalcError,
    lexer::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    pub fn parse(&mut self) -> Result<Expr, CalcError> {
        let expr = self.parse_expression(0)?;

        match self.current() {
            Token::End => Ok(expr),
            _ => Err(CalcError::UnexpectedToken),
        }
    }

    fn precedence(token: &Token) -> Option<u8> {
        match token {
            Token::Plus | Token::Minus => Some(1),
            Token::Mul | Token::Div => Some(2),
            Token::Pow => Some(3),
            _ => None,
        }
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expr, CalcError> {
        let mut left = self.parse_unary()?;

        loop {
            let prec = match Self::precedence(self.current()) {
                Some(p) if p >= min_prec => p,
                _ => break,
            };

            let op = match self.current() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                Token::Mul => BinaryOp::Mul,
                Token::Div => BinaryOp::Div,
                Token::Pow => BinaryOp::Pow,
                _ => unreachable!(),
            };

            self.advance();

            let right = self.parse_expression(
                if matches!(op, BinaryOp::Pow) {
                    prec
                } else {
                    prec + 1
                }
            )?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, CalcError> {
        match self.current() {
            Token::Plus => {
                self.advance();

                Ok(Expr::Unary {
                    op: UnaryOp::Pos,
                    expr: Box::new(self.parse_unary()?),
                })
            }

            Token::Minus => {
                self.advance();

                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(self.parse_unary()?),
                })
            }

            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, CalcError> {
        let mut expr = self.parse_primary()?;

        while matches!(self.current(), Token::Factorial) {
            self.advance();

            expr = Expr::Unary {
                op: UnaryOp::Factorial,
                expr: Box::new(expr),
            };
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, CalcError> {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }

            Token::LParen => {
                self.advance();

                let expr = self.parse_expression(0)?;

                match self.current() {
                    Token::RParen => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err(CalcError::UnexpectedToken),
                }
            }

            Token::Identifier(name) => {
                self.advance();

                match name.to_lowercase().as_str() {
                    "pi" => Ok(Expr::Constant(Constant::Pi)),
                    "e" => Ok(Expr::Constant(Constant::E)),

                    function => {
                        let function = match function {
                            "sin" => Function::Sin,
                            "cos" => Function::Cos,
                            "tan" => Function::Tan,
                            "asin" => Function::Asin,
                            "acos" => Function::Acos,
                            "atan" => Function::Atan,
                            "log" => Function::Log,
                            "ln" => Function::Ln,
                            "sqrt" => Function::Sqrt,
                            _ => {
                                return Err(
                                    CalcError::UnknownFunction(name)
                                )
                            }
                        };

                        match self.current() {
                            Token::LParen => self.advance(),
                            _ => return Err(CalcError::UnexpectedToken),
                        }

                        let argument = self.parse_expression(0)?;

                        match self.current() {
                            Token::RParen => self.advance(),
                            _ => return Err(CalcError::UnexpectedToken),
                        }

                        Ok(Expr::Function {
                            function,
                            argument: Box::new(argument),
                        })
                    }
                }
            }

            Token::End => Err(CalcError::UnexpectedEnd),

            _ => Err(CalcError::UnexpectedToken),
        }
    }
}
