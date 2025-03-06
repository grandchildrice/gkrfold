//! Module for sumfold protocol

pub mod fj_poly;
pub mod q_poly;
pub mod utils;

#[cfg(test)]
mod test;

use fj_poly::build_fj_polynomial;
use q_poly::build_Q_polynomial;
use ark_std::rand::Rng;
use std::sync::Arc;
use ark_ff::Field;
use ark_poly::DenseMultilinearExtension;

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
  pub instance: SumFoldInstance<F>,
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
  pub fn sumfold<R: Rng>(
      instances: Vec<SumFoldInstance<F>>,
      rng: &mut R,
  ) -> Self {
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

      let f_js: Vec<_> = gs.iter().map(|gs_for_j| build_fj_polynomial(gs_for_j)).collect();

      // Step 5: pick random rho in [0..n)
      let rho_int = rng.gen_range(0..n);

      let instance = instances[rho_int].clone();
      let mut sum_val = F::zero();
      for i in 0..x {
          sum_val += F_func(&instance.g_vec.iter().map(|g| g.evaluations[i]).collect::<Vec<_>>());
      }

      // Step 6: call build_q_polynomial
      let Q_poly = build_Q_polynomial(sum_val, rho_int, n);

      Self {
        Q_poly,
        fj_poly: f_js,
        instance,
      }
  }

  /// Verifies the SumFoldProof by following the requested steps:
  // 1. pick random index rho
  // 2. evaluate Ti from fj polys
  // 3. check if Ti = claim by sumcheck protocol
  // 4. apply sumcheck protocol
  // 5. commit to fj polys
  // 6. return Ti, commit(f1), commit(f2), ..., commit(ft)
  #[allow(non_snake_case)]
  pub fn verify<R: Rng>(&self, rng: &mut R,) -> bool {
    let F_func = self.instance.F_func.clone();
    let n = self.Q_poly.evaluations.len();
    let rho_int = rng.gen_range(0..n);
    let sum_val = F_func(&self.fj_poly.iter().map(|fj| fj.evaluations[rho_int]).collect::<Vec<_>>());
    sum_val == self.Q_poly.evaluations[rho_int]
    // TODO: change to return SumCheckProof and commitment to fj polys and claim
  }
}
