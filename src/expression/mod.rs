use num_rational::*;

pub mod differential;
pub mod evaluate;
pub mod matrix_expression;
pub mod operators;
pub mod rust_code;
pub mod symbol;
pub mod transcendental_expression;

pub use differential::*;
pub use evaluate::*;
pub use matrix_expression::*;
pub use rust_code::*;
pub use symbol::*;
pub use transcendental_expression::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Symbol(String),
    Constant(f64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Pow(Box<Expression>, Ratio<u32>),
    Transcendental(Box<TranscendentalExpression>),
    MatrixScalar(Box<MatrixExpression>),
}

impl From<f64> for Expression {
    fn from(v: f64) -> Self {
        Expression::Constant(v)
    }
}
