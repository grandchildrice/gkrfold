use super::*;
use crate::sumfold::utils::{build_random_poly, product_f};
use ark_std::{rand::rngs::StdRng, rand::SeedableRng, One};
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
                println!("test_sumfold n={} l={} t={}", n, l, t);
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
                let verified = SumFoldProof::verify(&proof, &mut verifier_rng).unwrap();
                assert!(verified, "Failed at n={} l={} t={}", n, l, t);
            }
        }
    }
}

#[test]
fn test_sumfold_soundness() {
    // n: num of instances
    let ns = [2, 4, 8, 16];
    // l: num of bits for g polynomial
    let ls = [2, 4, 8, 16];
    let num_tries = 10;

    for &n in &ns {
        for &l in &ls {
            for t in 0..num_tries {
                println!("test_sumfold n={} l={} t={}", n, l, t);
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
                let proof = SumFoldProof::sumfold(instances.clone(), &mut prover_rng);

                // verify with invalid rho
                let mut verifier_rng = StdRng::seed_from_u64(126);
                let verified = SumFoldProof::verify(&proof, &mut verifier_rng);
                assert!(
                    verified.is_err(),
                    "Should failed, but succeed at n={} l={} t={}",
                    n,
                    l,
                    t
                );

                // verify with invalid claim
                let mut prover_rng2 = StdRng::seed_from_u64(122);
                let rho_int = prover_rng2.gen_range(0..n);
                let invalid_q_poly = build_Q_polynomial(
                    proof.Q_poly.evaluations[rho_int] + FF::one(),
                    rho_int,
                    (n as f64).log2() as usize,
                );
                let invalid_proof = SumFoldProof {
                    Q_poly: invalid_q_poly,
                    fj_poly: proof.fj_poly.clone(),
                    instances: instances.clone(),
                    proof: proof.proof.clone(),
                };
                let mut verifier_rng = StdRng::seed_from_u64(122);
                let verified = SumFoldProof::verify(&invalid_proof, &mut verifier_rng);
                assert!(
                    verified.is_err(),
                    "Should failed, but succeed at n={} l={} t={}",
                    n,
                    l,
                    t
                );
            }
        }
    }
}
