use crate::{Expression, Size, TensorExpression};
use opensrdk_linear_algebra::{
    generate_rank_combinations, sparse::operations::kronecker_delta::KroneckerDelta, Tensor,
};
use std::{collections::HashMap, hash::Hash, iter::once};

type TermIndex = usize;
type RankIndex = usize; // TODO

fn next_char(c: char, count: usize) -> char {
    std::char::from_u32(c as u32 + count as u32).unwrap_or(c)
}

pub trait InnerProd {
    fn inner_prod(self, rank_combinations: &[HashMap<RankIndex, String>]) -> TensorExpression;
}

impl<I> InnerProd for I
where
    I: Iterator<Item = TensorExpression>,
{
    fn inner_prod(self, rank_combinations: &[HashMap<RankIndex, String>]) -> TensorExpression {
        // Flatten InnerProd
        let terms = self
            .zip(rank_combinations.iter())
            .flat_map(|(t, rank_combination)| {
                if let TensorExpression::InnerProd {
                    terms: t,
                    mut rank_combinations,
                } = t
                {
                    let not_1dimension_ranks = TensorExpression::not_1dimension_ranks_in_inner_prod(
                        &t,
                        &rank_combinations,
                    );

                    for (&rank, id) in rank_combination.iter() {
                        let term_index = not_1dimension_ranks[&rank];
                        rank_combinations[term_index].insert(rank, id.to_owned());
                    }

                    return t
                        .into_iter()
                        .zip(rank_combinations.into_iter())
                        .collect::<Vec<_>>();
                }

                vec![(t, rank_combination.clone())]
            })
            .collect::<Vec<_>>();

        // Merge Zero
        if terms
            .iter()
            .find(|&(t, _)| TensorExpression::Zero.eq(t))
            .is_some()
        {
            return TensorExpression::Zero;
        }

        // Merge KroneckerDeltas
        let deltas = terms
            .iter()
            .filter_map(|(t, _)| {
                if let TensorExpression::KroneckerDeltas(rank_pairs) = t {
                    Some(rank_pairs)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let not_deltas = terms
            .iter()
            .filter(|(t, _)| !matches!(t, TensorExpression::KroneckerDeltas(_)))
            .collect::<Vec<_>>();

        let flatten_deltas = deltas.into_iter().flatten().cloned().collect::<Vec<_>>();
        let merged_deltas = TensorExpression::KroneckerDeltas(flatten_deltas);

        let new_rank_combinations = once(HashMap::new())
            .chain(not_deltas.iter().map(|&(_, r)| r.clone()))
            .collect::<Vec<_>>();
        let new_terms = once(merged_deltas)
            .chain(not_deltas.iter().map(|(t, _)| t.clone()))
            .collect::<Vec<_>>();

        TensorExpression::InnerProd {
            terms: new_terms,
            rank_combinations: new_rank_combinations,
        }
    }
}

impl TensorExpression {
    pub fn inner_prod(self, rhs: TensorExpression, rank_pairs: &[[RankIndex; 2]]) -> Self {
        // Merge constant
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = &rhs {
                return TensorExpression::Constant(vl.clone().inner_prod(vr.clone(), rank_pairs));
            }
            if let TensorExpression::KroneckerDeltas(rank_pairs_r) = &rhs {
                return TensorExpression::Constant(
                    vl.mul_kronecker_deltas(
                        &rank_pairs_r
                            .iter()
                            .map(|rank_pair| KroneckerDelta(rank_pair[0], rank_pair[1]))
                            .collect::<Vec<_>>(),
                    ),
                );
            }
            if vl.total_size() == 1 {
                return TensorExpression::MulScalarLhs(
                    Expression::Constant(vl[&vec![0; vl.rank()]]).into(),
                    rhs.into(),
                );
            }
        }
        if let TensorExpression::Constant(vr) = &rhs {
            if let TensorExpression::KroneckerDeltas(rank_pairs_l) = &self {
                return TensorExpression::Constant(
                    vr.mul_kronecker_deltas(
                        &rank_pairs_l
                            .iter()
                            .map(|rank_pair| KroneckerDelta(rank_pair[0], rank_pair[1]))
                            .collect::<Vec<_>>(),
                    ),
                );
            }
            if vr.total_size() == 1 {
                return TensorExpression::MulScalarRhs(
                    self.into(),
                    Expression::Constant(vr[&vec![0; vr.rank()]]).into(),
                );
            }
        }
        // Merging Zero, KroneckerDeltas, InnerProds are done in InnerProd::inner_prod

        vec![self, rhs]
            .into_iter()
            .inner_prod(&generate_rank_combinations(rank_pairs))
    }
}

impl TensorExpression {
    pub(crate) fn diff_inner_prod(
        symbols: &[&str],
        v: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> Vec<TensorExpression> {
        let mut result = v[0]
            .differential(symbols)
            .into_iter()
            .map(|d| {
                once(d)
                    .chain(v[1..].iter().cloned())
                    .inner_prod(rank_combinations)
            })
            .collect::<Vec<_>>();

        for i in 1..v.len() {
            result
                .iter_mut()
                .zip(v[i].differential(symbols).into_iter())
                .for_each(|(r, d)| {
                    *r = r.clone()
                        + v[0..i]
                            .iter()
                            .cloned()
                            .chain(once(d))
                            .chain(v[i + 1..].iter().cloned())
                            .inner_prod(rank_combinations);
                });
        }

        result
    }

    pub(crate) fn rust_code_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
        parentheses: bool,
    ) -> String {
        todo!()
    }

    pub(crate) fn tex_code_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> String {
        let mut identifier = HashMap::<String, usize>::new();

        for i in 0..terms.len() {
            for (_, id) in rank_combinations[i].iter() {
                if !identifier.contains_key(id) {
                    identifier.insert(id.clone(), identifier.len());
                }
            }
        }

        let mut result = String::new();
        result.push_str(&format!(
            r"\sum_{{{}}}",
            identifier
                .iter()
                .map(|(_, l)| format!("{}", next_char('i', *l)))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        for i in 0..terms.len() {
            result.push_str(&format!(
                "{}_{{{}}}",
                terms[i].tex_code(),
                rank_combinations[i]
                    .iter()
                    .map(|(j, id)| format!("[{}] = {}", j, next_char('i', identifier[id])))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        format!("{{{}}}", result)
    }

    pub(crate) fn size_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> Vec<Size> {
        let max_rank = terms.iter().map(|vi| vi.sizes().len()).max().unwrap();
        let mut sizes = vec![Size::One; max_rank];

        for i in 0..terms.len() {
            let term_sizes = terms[i].sizes();

            for (rank, size) in term_sizes.iter().enumerate() {
                if sizes[rank].eq(&Size::Many) {
                    continue;
                }
                if let Some(_) = rank_combinations[i].get(&rank) {
                    continue;
                }
                sizes.insert(rank, size.clone());
            }
        }

        sizes
    }

    pub fn not_1dimension_ranks_in_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> HashMap<RankIndex, TermIndex> {
        let mut not_1dimension_ranks = HashMap::new();

        for i in 0..terms.len() {
            let term_sizes = terms[i].sizes();
            for (rank, size) in term_sizes.iter().enumerate() {
                if let Some(_) = rank_combinations[i].get(&rank) {
                    continue;
                }

                if *size != Size::One {
                    if not_1dimension_ranks.contains_key(&rank) {
                        panic!(
                            "Rank {} is not 1-dimension in terms[{}] and terms[{}]",
                            rank,
                            not_1dimension_ranks.get(&rank).unwrap(),
                            i
                        );
                    }
                    not_1dimension_ranks.insert(rank, i);
                }
            }
        }

        not_1dimension_ranks
    }
}
