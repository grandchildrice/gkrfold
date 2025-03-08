
## 1. Introduction

The explanation of MLE, Sumcheck is omitted here. Those who wish to study the details should refer to the following documents.

- https://eprint.iacr.org/2013/351.pdf
- https://jolt.a16zcrypto.com/background/sumcheck.html
- https://www.youtube.com/watch?v=lMo-MmJ7e_E
- https://eprint.iacr.org/2019/317.pdf
- https://www.youtube.com/watch?v=lMo-MmJ7e_E

### 1-2. GKR Protocol and its challenges.

The GKR proof system[GKR15,XZZ19], a prominent SNARK (Succinct Non-interactive ARgument of Knowledge) due to its efficient prover time, has been employed in systems such as [Expander](https://github.com/PolyhedraZK/Expander) and [Ceno](https://github.com/scroll-tech/ceno).

Although the prover can generate proofs in $O(N)$ time without committing to intermediate results or relying on FFTs, the verification time and proof size exhibit a complexity of $O(D\log N)$. Consequently, achieving efficient proof compression remains an important challenge.

### 1-2. Contributions

In this work, we propose GKRFold, a novel method that applies SumFold to efficiently compress $n$ instances of GKR proofs. The primary contributions of this study are as follows:

1. We demonstrate that SumFold can be effectively utilized to compress GKR instances, leading to substantial improvements in proof size and verification efficiency.
2. We show that GKRFold can compress $n$ GKR proofs, each comprising $d$ layers (amounting to a total of $6 \times n \times d$ Sumcheck instances), into a single Sumcheck instance.

This result ... (TODO)...