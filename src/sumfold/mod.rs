//! Module for sumfold protocol

pub mod fj_poly;
pub mod q_poly;
pub mod utils;

#[cfg(test)]
mod test;

use ark_ff::Field;
use ark_poly::DenseMultilinearExtension;
use ark_std::rand::Rng;
use fj_poly::build_fj_polynomial;
use q_poly::build_Q_polynomial;
use std::{rc::Rc, sync::Arc};

use crate::ml_sumcheck::{protocol::ListOfProductsOfPolynomials, MLSumcheck, Proof};

/// SumFoldInstance contains the information needed to construct a SumFoldProof.
/// It contains a function F_func that takes a slice of Scalars (e.g. g0(x), g1(x), ...) and returns a single Scalar.
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SumFoldInstance<F: Field> {
    /// A function that takes a slice of Scalars (e.g. g0(x), g1(x), ...) and returns a single Scalar.
    pub F_func: Arc<dyn Fn(&[F]) -> F + Send + Sync>,

    /// A vector of multilinear polynomials: e.g. [g0, g1, ...].
    pub g_vec: Vec<DenseMultilinearExtension<F>>,
}

/// SumFoldProof contains the information needed to verify a SumFoldProof.
/// It contains a Q polynomial, a vector of f_j polynomials, and a SumFoldInstance.
/// The Q polynomial is constructed from the sum of F(g_vec[rho]) for a random rho.
/// The f_j polynomials are constructed from the g_vec polynomials.
/// The SumFoldInstance contains the F_func and g_vec polynomials.
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct SumFoldProof<F: Field> {
    /// The verify function checks if the Q polynomial evaluates to F(f_j(rho)) for the random rho.
    pub Q_poly: DenseMultilinearExtension<F>,
    /// A vector of f_j polynomials constructed from the g_vec polynomials.
    pub fj_poly: Vec<DenseMultilinearExtension<F>>,
    /// The SumFoldInstance contains the F_func and g_vec polynomials.
    pub instances: Vec<SumFoldInstance<F>>,
    /// The SumCheck Proof for the folded instance
    pub proof: Proof<F>,
}

impl<F: Field> SumFoldProof<F> {
    /// Implements sumfold() following the requested steps:
    /// 1. F = instances[0].f_func
    /// 2. Ensure all g_vec have the same length
    /// 3. define n, t, x, etc.
    /// 4. Prepare f_js from g_bj
    /// 5. receive random rho from Verifier
    /// 6. calculate sum_val = \sum F(g_vec[rho])
    /// 7. call build_Q_polynomial
    /// 8. return Q_poly, fj_poly, instance
    ///
    /// Output type: SumFoldProof<F>
    #[allow(non_snake_case)]
    pub fn sumfold<R: Rng>(instances: Vec<SumFoldInstance<F>>, rng: &mut R) -> Self {
        // Step 1: F = instances[0].f_func
        let F_func = instances[0].F_func.clone();

        // Step 2: Ensure all g_vec have the same length
        let first_len = instances[0].g_vec.len();
        for inst in &instances {
            assert_eq!(
                inst.g_vec.len(),
                first_len,
                "All instances must have the same number of polynomials in g_vec"
            );
        }

        // Step 3: define n, t, x, etc.
        let n = instances.len();
        let t = instances[0].g_vec.len();
        let x = instances[0].g_vec[0].evaluations.len();

        // Step 4: Prepare f_js from g_bj
        let gs: Vec<Vec<_>> = (0..t)
            .map(|j| instances.iter().map(|inst| inst.g_vec[j].clone()).collect())
            .collect();

        assert_eq!(gs.len(), t, "gs must have t elements");

        let f_js: Vec<_> = gs
            .iter()
            .map(|gs_for_j| build_fj_polynomial(gs_for_j))
            .collect();

        // Step 5: pick random rho in [0..n)
        let rho_int = rng.gen_range(0..n);

        let instance = instances[rho_int].clone();
        let mut sum_val = F::zero();
        for i in 0..x {
            let g_vals: Vec<_> = instance.g_vec.iter().map(|g| g.evaluations[i]).collect();
            sum_val += F_func(&g_vals);
        }

        // Step 6: call build_q_polynomial
        let nu = (n as f64).log2() as usize;
        let Q_poly = build_Q_polynomial(sum_val, rho_int, nu);

        Self {
            Q_poly,
            fj_poly: f_js.clone(),
            instances: instances.clone(),
            proof: Self::prove_outer_sumcheck(instances[rho_int].g_vec.clone()),
        }
    }

    /// Generates a ListOfProductsOfPolynomials from a vector of DenseMultilinearExtension
    pub fn generate_list_of_poly(
        g_vec: Vec<DenseMultilinearExtension<F>>,
    ) -> ListOfProductsOfPolynomials<F> {
        let dim = g_vec[0].num_vars;
        let mut poly: ListOfProductsOfPolynomials<F> = ListOfProductsOfPolynomials::new(dim);
        poly.add_product(
            g_vec.into_iter().map(|poly| Rc::new(poly.clone())),
            F::one(),
        );
        poly
    }

    /// Proves the folded instance by SumCheck Protocol
    pub fn prove_outer_sumcheck(g_vec: Vec<DenseMultilinearExtension<F>>) -> Proof<F> {
        let poly = Self::generate_list_of_poly(g_vec);
        MLSumcheck::prove(&poly).unwrap()
    }

    /// Verifies the outer SumCheck Protocol
    pub fn verify_outer_sumcheck(
        g_vec: Vec<DenseMultilinearExtension<F>>,
        proof: Proof<F>,
        claim: F,
    ) -> bool {
        let poly = Self::generate_list_of_poly(g_vec);
        MLSumcheck::verify(&poly.info(), claim, &proof).is_ok()
    }

    /// Verifies the SumFoldProof by following the requested steps:
    // 1. pick random index rho
    // 2. evaluate Ti from fj polys
    // 3. check if Ti = claim by sumcheck protocol
    // 4. apply sumcheck protocol
    // 5. commit to fj polys
    // 6. return Ti, commit(f1), commit(f2), ..., commit(ft)
    #[allow(non_snake_case)]
    pub fn verify<R: Rng>(&self, rng: &mut R) -> Result<bool, Error> {
        let F_func = self.instances[0].F_func.clone();
        let n = self.Q_poly.evaluations.len();

        // Important: Use the same rho_int as in sumfold
        // The test uses the same seed for both prover and verifier
        let rho_int = rng.gen_range(0..n);

        // Calculate the sum of F(f_1(rho_int, x), ..., f_t(rho_int, x)) over all possible values of x
        let nu = (n as f64).log2() as usize; // Number of variables for b
        let l = self.fj_poly[0].num_vars - nu; // Number of variables for x
        let x_size = 1 << l; // Number of possible values for x

        let mut sum_val = F::zero();
        for x_val in 0..x_size {
            // Calculate the index in the evaluations array for (rho_int, x_val)
            let index = (rho_int << l) | x_val;

            // Extract the values of f_j(rho_int, x_val) for all j
            let f_vals: Vec<_> = self
                .fj_poly
                .iter()
                .map(|fj| fj.evaluations[index])
                .collect();

            // Apply F to these values and add to the sum
            sum_val += F_func(&f_vals);
        }

        for i in 0..self.Q_poly.evaluations.len() {
            if i == rho_int {
                if self.Q_poly.evaluations[i] != sum_val {
                    return Err(Error::InvalidRho);
                }
                if sum_val.is_zero() {
                    return Err(Error::InvalidSum);
                }
            } else {
                if !self.Q_poly.evaluations[i].is_zero() {
                    return Err(Error::InvalidRho);
                }
            }
        }

        if sum_val != self.Q_poly.evaluations[rho_int] {
            return Err(Error::InvalidSum);
        }

        if !Self::verify_outer_sumcheck(
            self.instances[rho_int].g_vec.clone(),
            self.proof.clone(),
            sum_val,
        ) {
            return Err(Error::InvalidSum);
        }

        Ok(true)
    }
}

/// Error type for SumFoldProof
#[derive(Debug)]
pub enum Error {
    /// The sum value is invalid
    InvalidSum,
    /// The rho value is invalid
    InvalidRho,
}
