#[macro_use]
extern crate criterion;

use ark_ff::Field;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
use ark_std::ops::Range;
use ark_std::test_rng;
use criterion::{black_box, BenchmarkId, Criterion};
use gkrfold::{
    ml_sumcheck::{protocol::ListOfProductsOfPolynomials, MLSumcheck},
    sumfold::{
        utils::{build_random_poly, product_f},
        SumFoldInstance, SumFoldProof,
    },
};
use std::{rc::Rc, sync::Arc};

const NUM_INSTANCE_RANGE: Range<usize> = 1..9;
const L: usize = 16;

fn prove_bench<F: Field>(c: &mut Criterion) {
    let mut rng = test_rng();

    let mut group = c.benchmark_group("MultiProve");
    for log_n in NUM_INSTANCE_RANGE {
        let n = 1 << log_n;
        group.bench_with_input(BenchmarkId::new("SumFold", n), &n, |b, &n| {
            let mut instances = Vec::with_capacity(n);
            for _ in 0..n {
                // build g0,g1
                let g0 = DenseMultilinearExtension::<F>::rand(L, &mut rng);
                let g1 = DenseMultilinearExtension::<F>::rand(L, &mut rng);
                // store
                let inst = SumFoldInstance {
                    F_func: Arc::new(product_f),
                    g_vec: vec![g0, g1],
                };
                instances.push(inst);
            }
            b.iter(|| SumFoldProof::sumfold(black_box(instances.clone()), black_box(&mut rng)));
        });

        group.bench_with_input(BenchmarkId::new("SumCheck", n), &n, |b, &n| {
            let product: Vec<_> = (0..3)
                .map(|_| Rc::new(DenseMultilinearExtension::<F>::rand(L, &mut rng)))
                .collect();
            let coefficient = F::rand(&mut rng);
            let mut products = ListOfProductsOfPolynomials::new(L);
            products.add_product(product, coefficient);
            b.iter(|| {
                for _ in 0..n {
                    MLSumcheck::prove(black_box(&products)).unwrap();
                }
            });
        });
    }

    group.finish();
}

fn verify_bench<F: Field>(c: &mut Criterion) {
    let mut rng = test_rng();

    let mut group = c.benchmark_group("MultiVerify");
    for log_n in NUM_INSTANCE_RANGE {
        let n = 1 << log_n;
        group.bench_with_input(BenchmarkId::new("SumFold", n), &n, |b, &n| {
            let mut instances = Vec::with_capacity(n);
            for _ in 0..n {
                // build g0,g1
                let g0: DenseMultilinearExtension<F> = build_random_poly(L, &mut rng);
                let g1: DenseMultilinearExtension<F> = build_random_poly(L, &mut rng);
                // store
                let inst = SumFoldInstance {
                    F_func: Arc::new(product_f),
                    g_vec: vec![g0, g1],
                };
                instances.push(inst);
            }
            let mut prover_rng = test_rng();
            let proof = SumFoldProof::sumfold(instances.clone(), &mut prover_rng);

            let mut verifier_rng = test_rng();
            b.iter(|| SumFoldProof::verify(black_box(&proof), black_box(&mut verifier_rng)));
        });

        group.bench_with_input(BenchmarkId::new("SumCheck", n), &n, |b, &n| {
            let product: Vec<_> = (0..3)
                .map(|_| Rc::new(DenseMultilinearExtension::<F>::rand(L, &mut rng)))
                .collect();
            let coefficient = F::rand(&mut rng);
            let mut products = ListOfProductsOfPolynomials::new(L);
            products.add_product(product, coefficient);
            let proof = MLSumcheck::prove(&products).unwrap();
            let expected_sum = MLSumcheck::extract_sum(&proof);

            b.iter(|| {
                for _ in 0..n {
                    MLSumcheck::verify(&products.info(), black_box(expected_sum), &proof).unwrap();
                }
            });
        });
    }

    group.finish();
}

fn bench_bls_381(c: &mut Criterion) {
    prove_bench::<ark_test_curves::bls12_381::Fr>(c);
    verify_bench::<ark_test_curves::bls12_381::Fr>(c);
}

criterion_group!(benches, bench_bls_381);
criterion_main!(benches);
