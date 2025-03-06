use super::*;
use crate::sumfold::utils::build_random_poly;
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_test_curves::bls12_381::Fr as FF;

fn random_gkr_instance<F: Field, R: Rng>(
    dim: usize,
    rng: &mut R,
) -> (
    SparseMultilinearExtension<F>,
    DenseMultilinearExtension<F>,
    DenseMultilinearExtension<F>,
) {
    (
        SparseMultilinearExtension::rand_with_config(dim * 3, 1 << dim, rng),
        build_random_poly(dim, rng),
        build_random_poly(dim, rng),
    )
}

#[test]
fn test_gkrfold_correctness() {
    let ns = [2, 4, 8, 16];
    let ls = [2, 4, 8, 16];
    let num_tries = 10;
    for &n in &ns {
        for &l in &ls {
            for t in 0..num_tries {
                println!("test_gkrfold n={} l={} t={}", n, l, t);
                let mut rng = StdRng::seed_from_u64(99);
                let mut instances = Vec::with_capacity(n);
                for _ in 0..n {
                    let (f1, f2, f3) = random_gkr_instance(l, &mut rng);
                    let g: Vec<_> = (0..l).map(|_| FF::rand(&mut rng)).collect();

                    let inst = GKRFoldInstance {
                        f1: f1.clone(),
                        f2: f2.clone(),
                        f3: f3.clone(),
                        g: g.clone(),
                    };
                    instances.push(inst);
                }

                let mut sumfold_rng = StdRng::seed_from_u64(122);
                let gkrfold_proof = gkrfold(instances, &mut rng, &mut sumfold_rng);

                let mut verifier_rng = StdRng::seed_from_u64(122);
                let verified = SumFoldProof::verify(&gkrfold_proof, &mut verifier_rng).unwrap();
                assert!(verified, "Failed at n={} l={} t={}", n, l, t);
            }
        }
    }
}
