//! This module contains the implementation of the GKRFold protocol.
//! The GKRFold protocol is a generalization of the GKR protocol that allows for folding of the
//! GKR protocol to reduce the number of rounds.

use std::sync::Arc;

use ark_ff::{Field, Zero};
use ark_poly::{DenseMultilinearExtension, Polynomial, SparseMultilinearExtension};
use ark_std::rand::Rng;

#[cfg(test)]
mod test;

use crate::sumfold::{SumFoldProof, SumFoldInstance, utils::product_f};
use crate::gkr_round_sumcheck::{initialize_phase_one, initialize_phase_two};

/// A GKRFold instance is a tuple of three multilinear extensions and a vector of field elements.
pub struct GKRFoldInstance<F: Field> {
    /// f1 is a multilinear extension of a polynomial in 3n variables.
    pub f1: SparseMultilinearExtension<F>,
    /// f2 is a multilinear extension of a polynomial in n variables.
    pub f2: DenseMultilinearExtension<F>,
    /// f3 is a multilinear extension of a polynomial in n variables.
    pub f3: DenseMultilinearExtension<F>,
    /// g is a vector of field elements of length n.
    pub g: Vec<F>,
}

/// The `gkrfold` function constructs a SumFoldProof from a vector of GKRFold instances.
/// The SumFoldProof contains a Q polynomial, a vector of f_j polynomials, and a SumFoldInstance.
#[allow(non_snake_case)]
pub fn gkrfold<F: Field, R: Rng>(
  instances: Vec<GKRFoldInstance<F>>,
  rng: &mut R,
  sumfold_rng: &mut R,
) -> SumFoldProof<F> {
  let mut sc_instances = Vec::new();
  let mut u = vec![F::zero(); instances[0].f2.num_vars];
  let mut v = vec![F::one(); instances[0].f2.num_vars];
  let F_func = Arc::new(product_f);

  for instance in &instances {
    let (f1, f2, f3, g) = (instance.f1.clone(), instance.f2.clone(), instance.f3.clone(), instance.g.clone());
    let dim = f2.num_vars;

    // initialize phase one and get h_g, f1_g
    let (h_g, f1_g) = initialize_phase_one(&f1, &f3, &g);

    // initialize phase two and get f1_gu
    let f1_gu = initialize_phase_two(&f1_g, &u);

    // calculate f3(f2(u))
    let f3_f2u = {
      let mut zero = DenseMultilinearExtension::zero();
      zero += (f2.evaluate(&u), &f3);
      zero
    };

    // prepare SumFoldInstance
    let sc_1 = SumFoldInstance {
      F_func: F_func.clone(),
      g_vec: vec![h_g, f2],
    };
    let sc_2 = SumFoldInstance {
      F_func: F_func.clone(),
      g_vec: vec![f1_gu, f3_f2u],
    };
    sc_instances.push(sc_1);
    sc_instances.push(sc_2);

    // receive new random (u,v) from Verifier
    u = (0..dim).map(|_| F::rand(rng)).collect::<Vec<F>>();
    v = (0..dim).map(|_| F::rand(rng)).collect::<Vec<F>>();
  }

  SumFoldProof::sumfold(sc_instances, sumfold_rng)
}