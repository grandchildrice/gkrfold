//! This file is part of the SumFold library.
use ark_ff::Field;
use ark_poly::DenseMultilinearExtension;

#[cfg(test)]
use ark_std::{
    rand::rngs::StdRng,
    rand::{Rng, SeedableRng},
};
#[cfg(test)]
use ark_test_curves::bls12_381::Fr as FF;

/// Constructs Q(b) = eq(rho, b) * ( sum_{x in {0,1}^l} F( f_1(b,x), ..., f_t(b,x) ) ).
///
/// - `T`: The value to be placed at index `rho` in the polynomial
/// - `rho`: The index where the polynomial should evaluate to `T` (all other indices evaluate to 0)
/// - `n`: The number of variables in the resulting polynomial (log2 of the number of evaluations)
///
/// Returns a DenseMultilinearExtension in `n` variables that evaluates to `T` at index `rho` and 0 elsewhere.
#[allow(non_snake_case)]
pub fn build_Q_polynomial<F: Field>(T: F, rho: usize, n: usize) -> DenseMultilinearExtension<F> {
    let bit_size = 1 << n;
    let mut evals = vec![F::zero(); bit_size];
    evals[rho] = T;
    // Create a DenseMultilinearExtension directly from the evaluations
    DenseMultilinearExtension::from_evaluations_vec(n, evals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Zero;

    #[allow(non_snake_case)]
    fn test_build_Q_polynomial_simple(n: usize) {
        println!("test_build_Q_polynomial_simple n={}", n);
        let nu = (n as f64).log2() as usize;
        let mut rng = StdRng::seed_from_u64(111);

        // Sample a random rho in [0..n)
        let random_u64 = rng.gen::<u64>();
        let rho = (random_u64 as usize) % n;

        println!("rho: {}", rho);

        // Generate a random value for T
        let T = FF::from(rng.gen::<u64>());

        // Build the Q polynomial
        let Q = build_Q_polynomial(T, rho, nu);

        // Verify that Q evaluates to T at rho and 0 elsewhere
        for i in 0..n {
            if i == rho {
                assert_eq!(Q.evaluations[i], T, "Q should evaluate to T at rho");
            } else {
                assert_eq!(
                    Q.evaluations[i],
                    FF::zero(),
                    "Q should evaluate to 0 elsewhere"
                );
            }
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_build_Q_poly() {
        for _ in 0..10 {
            test_build_Q_polynomial_simple(2);
            test_build_Q_polynomial_simple(8);
            test_build_Q_polynomial_simple(16);
        }
    }
}
