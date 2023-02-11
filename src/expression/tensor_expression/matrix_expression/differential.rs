use crate::{MatrixExpression, TensorExpression};

impl MatrixExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<TensorExpression> {
        match self {
            MatrixExpression::Mat(v) => v.differential(symbols),
            MatrixExpression::Constant(_) => vec![],
            MatrixExpression::T(v) => MatrixExpression::diff_t(v, symbols),
            MatrixExpression::Inv(v) => MatrixExpression::diff_inv(v, symbols),
            MatrixExpression::Det(v) => MatrixExpression::diff_det(v, symbols),
        }
    }
}
