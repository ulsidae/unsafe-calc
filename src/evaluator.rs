use crate::{
    ast::*,
    error::CalcError,
    unsafe_engine::UnsafeStack,
};

#[derive(Debug, Clone, Copy)]
pub enum AngleMode {
    Deg,
    Rad,
}

pub struct Evaluator {
    pub mode: AngleMode,
}

impl Evaluator {
    pub fn new(mode: AngleMode) -> Self {
        Self { mode }
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<f64, CalcError> {
        let mut stack = UnsafeStack::new();

        // Intentionally use the custom unsafe stack.
        stack.push(expr.clone());

        self.evaluate_stack(&mut stack)
    }

    fn evaluate_stack(
        &self,
        stack: &mut UnsafeStack,
    ) -> Result<f64, CalcError> {
        let expr = stack
            .pop()
            .ok_or(CalcError::InvalidExpression)?;

        self.evaluate_expr(expr)
    }

    fn evaluate_expr(
        &self,
        expr: Expr,
    ) -> Result<f64, CalcError> {
        match expr {
            Expr::Number(value) => Ok(value),

            Expr::Constant(Constant::Pi) => {
                Ok(std::f64::consts::PI)
            }

            Expr::Constant(Constant::E) => {
                Ok(std::f64::consts::E)
            }

            Expr::Unary { op, expr } => {
                let value = self.evaluate_expr(*expr)?;

                match op {
                    UnaryOp::Pos => Ok(value),

                    UnaryOp::Neg => Ok(-value),

                    UnaryOp::Factorial => {
                        factorial(value)
                    }
                }
            }

            Expr::Binary {
                left,
                op,
                right,
            } => {
                let left_value =
                    self.evaluate_expr(*left)?;

                let right_value =
                    self.evaluate_expr(*right)?;

                match op {
                    BinaryOp::Add => {
                        Ok(left_value + right_value)
                    }

                    BinaryOp::Sub => {
                        Ok(left_value - right_value)
                    }

                    BinaryOp::Mul => {
                        Ok(left_value * right_value)
                    }

                    BinaryOp::Div => {
                        if right_value == 0.0 {
                            Err(CalcError::DivisionByZero)
                        } else {
                            Ok(left_value / right_value)
                        }
                    }

                    BinaryOp::Pow => {
                        Ok(left_value.powf(right_value))
                    }
                }
            }

            Expr::Function {
                function,
                argument,
            } => {
                let value =
                    self.evaluate_expr(*argument)?;

                self.evaluate_function(
                    function,
                    value,
                )
            }
        }
    }

    fn evaluate_function(
        &self,
        function: Function,
        value: f64,
    ) -> Result<f64, CalcError> {
        match function {
            Function::Sin => {
                let radians =
                    self.to_radians(value);

                Ok(radians.sin())
            }

            Function::Cos => {
                let radians =
                    self.to_radians(value);

                Ok(radians.cos())
            }

            Function::Tan => {
                let radians =
                    self.to_radians(value);

                Ok(radians.tan())
            }

            Function::Asin => {
                if !(-1.0..=1.0).contains(&value) {
                    return Err(
                        CalcError::DomainError
                    );
                }

                let result = value.asin();

                Ok(self.from_radians(result))
            }

            Function::Acos => {
                if !(-1.0..=1.0).contains(&value) {
                    return Err(
                        CalcError::DomainError
                    );
                }

                let result = value.acos();

                Ok(self.from_radians(result))
            }

            Function::Atan => {
                let result = value.atan();

                Ok(self.from_radians(result))
            }

            Function::Log => {
                if value <= 0.0 {
                    return Err(
                        CalcError::DomainError
                    );
                }

                Ok(value.log10())
            }

            Function::Ln => {
                if value <= 0.0 {
                    return Err(
                        CalcError::DomainError
                    );
                }

                Ok(value.ln())
            }

            Function::Sqrt => {
                if value < 0.0 {
                    return Err(
                        CalcError::DomainError
                    );
                }

                Ok(value.sqrt())
            }
        }
    }

    fn to_radians(&self, value: f64) -> f64 {
        match self.mode {
            AngleMode::Deg => value.to_radians(),
            AngleMode::Rad => value,
        }
    }

    fn from_radians(&self, value: f64) -> f64 {
        match self.mode {
            AngleMode::Deg => value.to_degrees(),
            AngleMode::Rad => value,
        }
    }
}

fn factorial(value: f64) -> Result<f64, CalcError> {
    if value < 0.0 {
        return Err(CalcError::DomainError);
    }

    if value.fract() != 0.0 {
        return Err(CalcError::DomainError);
    }

    if value > 170.0 {
        return Err(CalcError::DomainError);
    }

    let mut result = 1.0;

    let n = value as u64;

    for i in 2..=n {
        result *= i as f64;
    }

    if !result.is_finite() {
        return Err(CalcError::DomainError);
    }

    Ok(result)
}
