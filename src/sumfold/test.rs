use crate::sumfold::utils::{build_random_poly, product_f};
use super::*;
use ark_std::{rand::SeedableRng, rand::rngs::StdRng};
use ark_test_curves::bls12_381::Fr as FF;

#[test]
fn test_sumfold_correctness() {
  // n: num of instances
  let ns = [2, 4, 8, 16];
  // l: num of bits for g polynomial
  let ls = [2, 4, 8, 16];
  let num_tries = 10;

  for &n in &ns {
    for &l in &ls {
      for t in 0..num_tries {
        println!("test_sumfold n={} l={} t={}",n,l,t);
        let mut rng = StdRng::seed_from_u64(99);

        // build 2 instances
        let mut instances = Vec::with_capacity(n);
        for _ in 0..n {
            // build g0,g1
            let g0: DenseMultilinearExtension<FF> = build_random_poly(l, &mut rng);
            let g1: DenseMultilinearExtension<FF> = build_random_poly(l, &mut rng);
            // store
            let inst = SumFoldInstance {
                F_func: Arc::new(product_f),
                g_vec: vec![g0, g1],
            };
            instances.push(inst);
        }

        // call sumfold
        let mut prover_rng = StdRng::seed_from_u64(122);
        let proof = SumFoldProof::sumfold(instances, &mut prover_rng);

        // verify
        let mut verifier_rng = StdRng::seed_from_u64(122);
        let verified = SumFoldProof::verify(&proof, &mut verifier_rng);
        assert!(verified, "Failed at n={} l={} t={}", n, l, t);
      }
    }
  }
}