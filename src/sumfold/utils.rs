//! This file contains utility functions for the sumfold crate

use ark_ff::Field;
use ark_poly::DenseMultilinearExtension;
use ark_std::rand::Rng;

/// Builds a random multilinear polynomial with n variables
pub fn build_random_poly<F: Field, R: Rng>(n: usize, rng: &mut R) -> DenseMultilinearExtension<F> {
  let size = 1 << n;

  DenseMultilinearExtension::from_evaluations_vec(
      n,
      (0..size)
          .into_iter()
          .map(|_| {
              F::from(rng.gen_range(1..50) as u64)
          })
          .collect()
  )
}

/// Computes the product of a slice of field elements
pub fn product_f<F: Field>(vals: &[F]) -> F {
  let mut acc = F::one();
  for &v in vals {
      acc *= v;
  }
  acc
}