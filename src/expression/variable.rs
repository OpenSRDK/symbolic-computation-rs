use crate::{Expression, Size, TensorExpression};
use std::{collections::HashSet, iter::once};

pub fn new_variable(id: String) -> Expression {
    Expression::Variable(id, vec![])
}

impl Expression {
    pub fn variable_ids(&self) -> HashSet<&str> {
        match self {
            Expression::Variable(id, _) => once(id.as_str()).collect::<HashSet<_>>(),
            Expression::Constant(_) => HashSet::new(),
            Expression::PartialVariable(v) => v
                .elems()
                .values()
                .into_iter()
                .flat_map(|v| v.variable_ids())
                .collect(),
            Expression::Add(l, r) => l
                .variable_ids()
                .into_iter()
                .chain(r.variable_ids().into_iter())
                .collect(),
            Expression::Sub(l, r) => l
                .variable_ids()
                .into_iter()
                .chain(r.variable_ids().into_iter())
                .collect(),
            Expression::Mul(l, r) => l
                .variable_ids()
                .into_iter()
                .chain(r.variable_ids().into_iter())
                .collect(),
            Expression::Div(l, r) => l
                .variable_ids()
                .into_iter()
                .chain(r.variable_ids().into_iter())
                .collect(),
            Expression::Neg(v) => v.variable_ids(),
            Expression::Transcendental(v) => v.variable_ids(),
            Expression::Tensor(v) => v.variable_ids(),
            Expression::Matrix(v) => v.variable_ids(),
        }
    }

    pub(crate) fn diff_variable(
        symbol: &String,
        sizes: &Vec<Size>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        let rank = sizes.len();
        variable_ids
            .iter()
            .map(|&s| {
                if s == symbol.as_str() {
                    if rank == 0 {
                        1.0.into()
                    } else {
                        TensorExpression::KroneckerDeltas(
                            (0..rank).map(|r| [r, r + rank]).collect(),
                        )
                        .into()
                    }
                } else {
                    0.0.into()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::new_variable;

    #[test]
    fn it_works() {
        let id = "x";
        let a = HashSet::from([id; 1]);
        let ea = new_variable((id).to_string());
        let ha = ea.variable_ids();

        assert_eq!(a, ha);
    }
    #[test]
    fn it_works2() {
        let x = new_variable("x".to_string());
        let mu = new_variable("mu".to_string());
        let sigma = new_variable("sigma".to_string());
        let expression = x * mu / sigma;
        let diff_x = expression.differential(&["x"])[0].clone();
        let diff_mu = expression.differential(&["mu"])[0].clone();
        let diff_sigma = expression.differential(&["sigma"])[0].clone();
        let diff_anpan = expression.differential(&["anpan"])[0].clone();

        let tex_symbols = vec![("x", "x"), ("mu", r"\mu"), ("sigma", r"\Sigma")]
            .into_iter()
            .collect();

        println!("{:#?}", diff_x.tex_code(&tex_symbols));
        println!("{:#?}", diff_mu.tex_code(&tex_symbols));
        println!("{:#?}", diff_sigma.tex_code(&tex_symbols));
        println!("{:#?}", diff_anpan.tex_code(&tex_symbols));
    }
}
