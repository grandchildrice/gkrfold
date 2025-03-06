#[macro_use]
extern crate criterion;

use ark_ff::Field;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, SparseMultilinearExtension};
use ark_std::ops::Range;
use criterion::{black_box, BenchmarkId, Criterion};
use gkrfold::{
    gkr_round_sumcheck::GKRRoundSumcheck,
    gkrfold::{gkrfold, GKRFoldInstance},
    rng::{Blake2b512Rng, FeedableRNG},
    sumfold::SumFoldProof,
};

const NUM_INSTANCE_RANGE: Range<usize> = 1..15;
const L: usize = 16;

fn prove_bench<F: Field>(c: &mut Criterion) {
    let mut group = c.benchmark_group("GKRMultiProve");
    for log_n in NUM_INSTANCE_RANGE {
        let n = 1 << log_n;
        group.bench_with_input(BenchmarkId::new("GKRFold", n), &n, |b, &n| {
            let mut rng = Blake2b512Rng::setup();
            let instances = (0..n)
                .map(|_| {
                    let f1 = SparseMultilinearExtension::rand_with_config(3 * L, 1 << L, &mut rng);
                    let f2 = DenseMultilinearExtension::rand(L, &mut rng);
                    let f3 = DenseMultilinearExtension::rand(L, &mut rng);
                    let g: Vec<_> = (0..L).map(|_| F::rand(&mut rng)).collect();

                    GKRFoldInstance {
                        f1: f1.clone(),
                        f2: f2.clone(),
                        f3: f3.clone(),
                        g: g.clone(),
                    }
                })
                .collect::<Vec<GKRFoldInstance<F>>>();
            let mut sumfold_rng = Blake2b512Rng::setup();
            b.iter(|| {
                gkrfold(
                    black_box(instances.clone()),
                    black_box(&mut rng),
                    black_box(&mut sumfold_rng),
                );
            });
        });

        group.bench_with_input(BenchmarkId::new("LinearGKR", n), &n, |b, &n| {
            let mut rng = Blake2b512Rng::setup();
            let f1 = SparseMultilinearExtension::rand_with_config(3 * L, 1 << L, &mut rng);
            let f2 = DenseMultilinearExtension::rand(L, &mut rng);
            let f3 = DenseMultilinearExtension::rand(L, &mut rng);
            let g: Vec<_> = (0..L).map(|_| F::rand(&mut rng)).collect();
            b.iter(|| {
                for _ in 0..n {
                    GKRRoundSumcheck::prove(
                        &mut rng,
                        black_box(&f1),
                        black_box(&f2),
                        black_box(&f3),
                        black_box(&g),
                    );
                }
            });
        });
    }

    group.finish();
}

fn verify_bench<F: Field>(c: &mut Criterion) {
    let mut group = c.benchmark_group("GKRMultiVerify");
    for log_n in NUM_INSTANCE_RANGE {
        let n = 1 << log_n;
        group.bench_with_input(BenchmarkId::new("GKRFold", n), &n, |b, &n| {
            let mut rng = Blake2b512Rng::setup();
            let instances = (0..n)
                .map(|_| {
                    let f1 = SparseMultilinearExtension::rand_with_config(3 * L, 1 << L, &mut rng);
                    let f2 = DenseMultilinearExtension::rand(L, &mut rng);
                    let f3 = DenseMultilinearExtension::rand(L, &mut rng);
                    let g: Vec<_> = (0..L).map(|_| F::rand(&mut rng)).collect();

                    GKRFoldInstance {
                        f1: f1.clone(),
                        f2: f2.clone(),
                        f3: f3.clone(),
                        g: g.clone(),
                    }
                })
                .collect::<Vec<GKRFoldInstance<F>>>();
            let mut sumfold_rng = Blake2b512Rng::setup();
            let proof = gkrfold(instances.clone(), &mut rng, &mut sumfold_rng);
            let mut verifier_rng = Blake2b512Rng::setup();
            b.iter(|| SumFoldProof::verify(black_box(&proof), black_box(&mut verifier_rng)));
        });

        group.bench_with_input(BenchmarkId::new("LinearGKR", n), &n, |b, &n| {
            let mut rng = Blake2b512Rng::setup();
            let f1 = SparseMultilinearExtension::rand_with_config(3 * L, 1 << L, &mut rng);
            let f2 = DenseMultilinearExtension::rand(L, &mut rng);
            let f3 = DenseMultilinearExtension::rand(L, &mut rng);
            let g: Vec<_> = (0..L).map(|_| F::rand(&mut rng)).collect();
            let proof = GKRRoundSumcheck::prove(&mut rng, &f1, &f2, &f3, &g);
            let expected_sum = proof.extract_sum();
            b.iter(|| {
                for _ in 0..n {
                    GKRRoundSumcheck::verify(&mut rng, f2.num_vars, &proof, expected_sum);
                }
            });
        });
    }

    group.finish();
}

fn bench_bls_381(c: &mut Criterion) {
    // prove_bench::<ark_test_curves::bls12_381::Fr>(c);
    verify_bench::<ark_test_curves::bls12_381::Fr>(c);
}

criterion_group!(benches, bench_bls_381);
criterion_main!(benches);
