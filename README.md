<h1 align="center">GKRFold</h1>

**WARNING**: This is an academic proof-of-concept prototype, and in particular has not received careful code review. This implementation is NOT ready for production use.

## Build guide

The library compiles on the `stable` toolchain of the Rust compiler. To install the latest version of Rust, first install `rustup` by following the instructions [here](https://rustup.rs/), or via your platform's package manager. Once `rustup` is installed, install the Rust toolchain by invoking:
```bash
rustup install stable
```

After that, use `cargo` (the standard Rust build tool) to build the library:
```bash
git clone <repo-url>
cd gkrfold
cargo build --release
```

This library comes with some unit and integration tests. Run these tests with:
```bash
cargo test
```

Lastly, this library is instrumented with profiling infrastructure that prints detailed traces of execution time. To enable this, compile with `cargo build --features print-trace`.

## Benchmarks

To run the benchmarks, install the nightly Rust toolchain, via `rustup install nightly`, and then run the following command:

```shell
cargo +nightly bench --all-features
```

or running specific bench:

```shell
cargo bench --bench ml_sumcheck
cargo bench --bench gkr_round_sumcheck
```

All benchmarks below are performed over BLS12-381 scalar field implemented in the `ark-test-curves` library. Benchmarks were run on a machine with an (TODO: hardware spec).

#### Benchmarks for `SumFold`

TODO

#### Benchmarks for `GKRFold`

TODO

## License

This library is licensed under either of the following licenses, at your discretion.

* [Apache License Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution that you submit to this library shall be dual licensed as above (as defined in the Apache v2 License), without any additional terms or conditions.

## Reference Paper
[Libra: Succinct Zero-Knowledge Proofs with Optimal Prover Computation](https://eprint.iacr.org/2019/317) <br/>
Tiancheng Xie, Jiaheng Zhang, Yupeng Zhang, Charalampos Papamanthou, Dawn Song

[Time-Optimal Interactive Proofs for Circuit Evaluation](https://arxiv.org/abs/1304.3812) <br/>
Justin Thaler

[NeutronNova: Folding everything that reduces to zero-check](https://eprint.iacr.org/2024/160) <br/>
Abhiram Kothapalli, Srinath Setty

[GKRFold: SumFold-based GKR Proof Compression](https://ethresear.ch/t/gkrfold-sumfold-based-gkr-proof-compression/21788/1) <br/>
Masato Tsutsumi

## Acknowledgement

This project would not have been possible without the invaluable resources and ideas provided by the following:

- **ark-sumcheck:** We forked the majority of the "sumcheck" and "gkr" code.
- **NeutronNova:** The SumFold idea provided great inspiration. Thank you.