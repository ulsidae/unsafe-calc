#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Constant(Constant),

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    Function {
        function: Function,
        argument: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Constant {
    Pi,
    E,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Pos,
    Factorial,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Log,
    Ln,
    Sqrt,
}
