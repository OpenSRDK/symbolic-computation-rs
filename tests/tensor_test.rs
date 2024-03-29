use opensrdk_symbolic_computation::{new_variable_tensor, Size};

extern crate blas_src;
extern crate lapack_src;
extern crate opensrdk_linear_algebra;
extern crate rayon;
extern crate ron;
extern crate serde;
extern crate thiserror;

mod probability;

#[test]
fn test_main() {
    let x = new_variable_tensor("x".to_owned(), vec![Size::Many]);
    let mu = new_variable_tensor("mu".to_owned(), vec![Size::Many]);
    let _lsigma = new_variable_tensor("lsigma".to_owned(), vec![Size::Many; 2]);
    let precision = new_variable_tensor("lambda".to_owned(), vec![Size::Many; 2]);

    let pdf_expression = (-0.5
        * ((x.clone() - mu.clone())
            .dot(precision, &[[0, 0]])
            .dot(x.clone() - mu.clone(), &[[1, 0]])))
    .exp();

    let diff = pdf_expression.differential(&["x", "mu", "lambda"]);
    let tex_symbols = vec![("x", "x"), ("mu", r"\mu"), ("lambda", r"\Lambda")]
        .into_iter()
        .collect();

    println!("x diff");
    println!("{:#?}", diff[0].tex_code(&tex_symbols));
    println!("mu diff");
    println!("{:#?}", diff[1].tex_code(&tex_symbols));
    println!("sigma diff");
    println!("{:#?}", diff[2].tex_code(&tex_symbols));
}

#[test]
fn test_main2() {
    let x = new_variable_tensor("x".to_owned(), vec![Size::Many, Size::One]);
    let mu = new_variable_tensor("mu".to_owned(), vec![Size::Many, Size::One]);
    let _lsigma = new_variable_tensor("lsigma".to_owned(), vec![Size::Many; 2]);
    let precision = new_variable_tensor("lambda".to_owned(), vec![Size::Many; 2]);

    let pdf_expression = (-0.5
        * ((x.clone() - mu.clone())
            .t()
            .dot(precision, &[[1, 0]])
            .dot(x.clone() - mu.clone(), &[[1, 0]])))
    .exp();

    let diff = pdf_expression.differential(&["x", "mu", "lambda"]);
    let tex_symbols = vec![("x", "x"), ("mu", r"\mu"), ("lambda", r"\Lambda")]
        .into_iter()
        .collect();

    println!("x diff");
    println!("{:#?}", diff[0].tex_code(&tex_symbols));
    println!("mu diff");
    println!("{:#?}", diff[1].tex_code(&tex_symbols));
    println!("sigma diff");
    println!("{:#?}", diff[2].tex_code(&tex_symbols));
}
