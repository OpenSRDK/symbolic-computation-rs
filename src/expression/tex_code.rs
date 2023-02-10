use crate::Expression;

impl Expression {
    pub fn tex_code(&self) -> String {
        match self {
            Expression::Symbol(symbol) => format!("{{{}}}", symbol),
            Expression::Constant(value) => format!("{}", value),
            Expression::Add(l, r) => format!("({} + {})", l.tex_code(), r.tex_code()),
            Expression::Sub(l, r) => format!("({} - {})", l.tex_code(), r.tex_code()),
            Expression::Mul(l, r) => format!("({} {})", l.tex_code(), r.tex_code()),
            Expression::Div(l, r) => format!("\\frac{{{}}}{{{}}}", l.tex_code(), r.tex_code()),
            Expression::Neg(v) => format!("-{}", v.tex_code()),
            Expression::Pow(base, exponent) => {
                format!("({}^{{{}}})", base.tex_code(), exponent.to_string())
            }
            Expression::Transcendental(v) => {
                format!("{}", v.tex_code())
            }
            Expression::TensorElement(v, indices) => format!(
                "{}_{{{}}}",
                v.tex_code(),
                indices
                    .iter()
                    .map(|i| (i + 1).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expression::_DiffResultTensor(v) => format!("{}", v.tex_code()),
        }
    }
}