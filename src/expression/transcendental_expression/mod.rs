pub mod assign;
pub mod differential;
pub mod functions;
pub mod size;
pub mod tex_code;
pub mod variable;

pub use assign::*;
pub use differential::*;
pub use size::*;
pub use tex_code::*;
pub use variable::*;

use crate::Expression;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TranscendentalExpression {
    Abs(Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
    Exp(Box<Expression>),
    Log(Box<Expression>, Box<Expression>),
    Ln(Box<Expression>),
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),
}

impl From<TranscendentalExpression> for Expression {
    fn from(t: TranscendentalExpression) -> Self {
        Expression::Transcendental(t.into())
    }
}
