use crate::{Expression, TensorExpression};
use std::ops::Mul;

fn mul_scalar(lhs: Expression, rhs: TensorExpression, swap: bool) -> TensorExpression {
    if let Expression::Constant(vl) = lhs {
        if let TensorExpression::Constant(vr) = rhs {
            return TensorExpression::Constant(vl * vr);
        }
        if vl == 0.0 {
            return TensorExpression::Zero;
        }
        if vl == 1.0 {
            return rhs;
        }
    }
    if let TensorExpression::Zero = rhs {
        return TensorExpression::Zero;
    }

    if swap {
        return TensorExpression::MulScalarRhs(rhs.into(), lhs.into());
    }
    TensorExpression::MulScalarLhs(lhs.into(), rhs.into())
}

impl Mul<TensorExpression> for Expression {
    type Output = TensorExpression;

    fn mul(self, rhs: TensorExpression) -> Self::Output {
        mul_scalar(self, rhs, false)
    }
}

impl Mul<Expression> for TensorExpression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        mul_scalar(rhs, self, true)
    }
}

impl Mul<TensorExpression> for f64 {
    type Output = TensorExpression;

    fn mul(self, rhs: TensorExpression) -> Self::Output {
        TensorExpression::MulScalarLhs(Expression::Constant(self).into(), rhs.into())
    }
}

impl Mul<f64> for TensorExpression {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        TensorExpression::MulScalarRhs(self.into(), Expression::Constant(rhs).into())
    }
}

impl TensorExpression {
    pub(crate) fn diff_mul_scalar_lhs(
        symbols: &[&str],
        l: &Box<Expression>,
        r: &Box<TensorExpression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }

    pub(crate) fn diff_mul_scalar_rhs(
        symbols: &[&str],
        l: &Box<TensorExpression>,
        r: &Box<Expression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }

    pub(crate) fn rust_code_mul_scalar_lhs(
        l: &Box<Expression>,
        r: &Box<TensorExpression>,
        parentheses: bool,
    ) -> String {
        if parentheses {
            format!("({} * {})", l._rust_code(true), r._rust_code(true))
        } else {
            format!("{} * {}", l._rust_code(true), r._rust_code(true))
        }
    }

    pub(crate) fn rust_code_mul_scalar_rhs(
        l: &Box<TensorExpression>,
        r: &Box<Expression>,
        parentheses: bool,
    ) -> String {
        if parentheses {
            format!("({} * {})", l._rust_code(true), r._rust_code(true))
        } else {
            format!("{} * {}", l._rust_code(true), r._rust_code(true))
        }
    }
}
