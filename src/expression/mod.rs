pub mod assign;
pub mod differential;
pub mod matrix_expression;
pub mod operators;
pub mod size;
pub mod symbol;
pub mod tensor_expression;
pub mod tex_code;
pub mod transcendental_expression;

pub use assign::*;
pub use differential::*;
pub use matrix_expression::*;
use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix};
pub use size::*;
pub use symbol::*;
pub use tensor_expression::*;
pub use tex_code::*;
pub use transcendental_expression::*;

use crate::ConstantValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Symbol(String, Vec<Size>),
    Constant(ConstantValue),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Transcendental(Box<TranscendentalExpression>),
    Tensor(Box<TensorExpression>),
    Matrix(Box<MatrixExpression>),
    Index(Box<Expression>, Vec<usize>),
    IndexedTensor(HashMap<Vec<usize>, Expression>),
}

impl From<f64> for Expression {
    fn from(v: f64) -> Self {
        Expression::Constant(ConstantValue::Scalar(v))
    }
}

impl From<SparseTensor> for Expression {
    fn from(v: SparseTensor) -> Self {
        Expression::Constant(ConstantValue::Tensor(v))
    }
}

impl From<Matrix> for Expression {
    fn from(v: Matrix) -> Self {
        Expression::Constant(ConstantValue::Matrix(v))
    }
}

impl From<ConstantValue> for Expression {
    fn from(v: ConstantValue) -> Self {
        match v {
            ConstantValue::Scalar(v) => v.into(),
            ConstantValue::Matrix(v) => v.into(),
            ConstantValue::Tensor(v) => v.into(),
        }
    }
}
