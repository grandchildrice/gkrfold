//! This module contains functions to construct and evaluate the multilinear extension (MLE) of a
//! set of polynomials for the j-th index. The MLE is defined as:
//!  f_j(b,x) = Σ_{i in {0,1}^ν} eq(b, i) * g_{i,j}(x)
use ark_ff::Field;
use ark_poly::DenseMultilinearExtension;

/// Given a set of polynomials for the j-th index:
/// g_{0,j}(x), g_{1,j}(x), ..., g_{(2^ν - 1), j}(x),
/// this function constructs and returns the multilinear extension (MLE)
/// defined by:
///   f_j(b,x) = Σ_{i in {0,1}^ν} eq(b, i) * g_{i,j}(x)
/// as an MLE in (ν + m) variables.
///
/// - gs_for_j: A slice of length 2^ν, where each element is a MultilinearPolynomial in m variables.
/// - Returns: A MultilinearPolynomial in (ν + m) variables (dense representation).
///
/// This implementation uses the `concat` function from `DenseMultilinearExtension` to efficiently
/// construct the polynomial. The `concat` function creates a new polynomial with one more variable
/// than the input polynomials, where the new variable selects between the different input polynomials.
/// By repeatedly applying `concat`, we can build a polynomial that selects between 2^ν different
/// polynomials based on the ν bits of the variable b.
pub fn build_fj_polynomial<F: Field>(
    gs_for_j: &[DenseMultilinearExtension<F>],
) -> DenseMultilinearExtension<F> {
    let num_b = gs_for_j.len();
    let nu = (num_b as f64).log2() as usize;
    assert_eq!(1 << nu, num_b, "gs_for_j.len() must be 2^ν");

    let l = gs_for_j[0].num_vars;
    for b in 1..num_b {
        assert_eq!(
            gs_for_j[b].num_vars, l,
            "all g_{{b,j}}(x) must have the same number of variables"
        );
    }

    // Use the concat function to build the polynomial
    // The concat function takes a collection of polynomials and returns a new polynomial
    // with one more variable than the input polynomials, where the new variable selects
    // between the different input polynomials.
    DenseMultilinearExtension::concat(gs_for_j)
}

/// Given decimal representations of b and x,
/// this function converts them internally into their bit representation (B1,...,Bν, X1,...,Xm)
/// and evaluates f_j(b,x).
///
/// - f: A MultilinearPolynomial in (ν + m) variables (constructed via build_fj_polynomial)
/// - b: Decimal representation of b. An integer with ν bits (0 <= b < 2^ν)
/// - x: Decimal representation of x. An integer with m bits (0 <= x < 2^m)
/// - nu: The number of bits required to represent b.
/// - m: The number of bits required to represent x.
pub fn evaluate_fj<F: Field>(f: &DenseMultilinearExtension<F>, b: usize, x: usize, l: usize) -> F {
    // For the specific case of evaluating f_j at (b, x), we can directly access the evaluation
    // at the corresponding index in the evaluations vector.
    // The index is calculated as (b << l) | x.
    let index = (b << l) | x;
    f.evaluations[index]
}

/// Test function for build_fj_polynomial
#[cfg(test)]
mod tests {
    use crate::sumfold::utils::build_random_poly;

    use super::*;
    use ark_std::{rand::rngs::StdRng, rand::SeedableRng};
    use ark_test_curves::bls12_381::Fr as FF;

    #[test]
    fn test_build_fj_polynomial_correctness() {
        test_build_fj_polynomial(2, 4);
        test_build_fj_polynomial(4, 4);
        test_build_fj_polynomial(8, 8);
        test_build_fj_polynomial(8, 4);
    }

    fn test_build_fj_polynomial(n: usize, x: usize) {
        println!("test_build_fj_polynomial n={} x={}", n, x);
        let nu = (n as f64).log2() as usize;
        let l = (x as f64).log2() as usize;
        let mut rng = StdRng::seed_from_u64(99);

        let gs_for_j: Vec<DenseMultilinearExtension<FF>> =
            (0..n).map(|_| build_random_poly(l, &mut rng)).collect();

        let f_j = build_fj_polynomial(&gs_for_j);
        assert_eq!(f_j.num_vars, nu + l);
        for bv in 0..n {
            for xv in 0..x {
                let expect = gs_for_j[bv].evaluations[xv];
                let actual = evaluate_fj(&f_j, bv, xv, l);
                assert_eq!(actual, expect, "Failed at bv={}, xv={}", bv, xv);
            }
        }
    }
}
