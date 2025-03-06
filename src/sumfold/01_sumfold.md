This explains SumFold, which folds n SumCheck instance into single one.

## 1. Introduction

### 1.1. NeutronNova and SumFold

SumFold, as introduced in NeutronNova[KS24], is a technique that folds an arbitrary number of Sumcheck instances into a single instance.

## 2. SumFold: High-Level Idea

The explanation of MLE, Sumcheck, and GKR is omitted here. Those who wish to study the details should refer to the following documents.

- https://eprint.iacr.org/2013/351.pdf
- https://jolt.a16zcrypto.com/background/sumcheck.html
- https://www.youtube.com/watch?v=lMo-MmJ7e_E
- https://eprint.iacr.org/2019/317.pdf
- https://www.youtube.com/watch?v=lMo-MmJ7e_E

Assume that we wish to prove, via Sumcheck, a function defined by a composition
$$
F(\vec{g}(x)),
$$
where $F$ is a polynomial in $t$ variables and $\vec{g}$ is a vector of $t$ polynomials in $l$ variables. For example, one may consider the case
$$
F(\vec{g}(x)) = g_0(x) \cdot g_1(x) \cdots g_{t-1}(x),
$$
which represents the product of the components of $\vec{g}(x)$.

The goal of SumFold is to reduce the proofs of $n$ Sumcheck instances
$$
[T_i = \sum_x F(\vec{g}(x))]_{i \in [n]}
$$
to a single Sumcheck proof.

![image](https://hackmd.io/_uploads/HyGj7mWc1g.png)

We define a Sumcheck instance $\mathsf{SC}$ that contains all the information required for a Sumcheck proof as follows:

![Screenshot from 2025-02-16 11-17-05](https://hackmd.io/_uploads/BykAY3k5yg.png)

SumFold folds the $n$ instances of $\mathsf{SC}$ into one. The main idea behind SumFold consists of the following steps:

1. For all $i = 1,\ldots,n$, given
   $$
   T_i = \sum_x F(\vec{g}_i(x)),
   $$
   construct a polynomial $Q(b)$ by interpolation such that, for any input $b$,
   $$
   Q(b) = T_b.
   $$
   This is achieved as follows:
   - (a) Construct $t$ polynomials $f_j(b,x)$ via polynomial interpolation, where each polynomial outputs $g_{bj}(x)$ corresponding to the $b$th instance.
   - (b) Reconstruct $Q(b)$ by applying the function $F$ to the collection $\{f_1(b,x), \dots, f_t(b,x)\}$.
2. Commit to the polynomial $Q(b)$. This commitment effectively aggregates the commitments for all $n$ Sumcheck instances.
3. The verifier selects a random challenge $r_b$ and designates the $r_b$th Sumcheck instance as the folded instance.

![image](https://hackmd.io/_uploads/BkyoQGGcJe.png)

It is verified (see Appendix B of the NeutronNova paper[KS24]) that
$$
Q(r_b) = T_{r_b}.
$$

### 3. SumFold: Protocol

Prover:
1. Prepare f_js from g_bj
2. receive random rho from Verifier
3. calculate sum_val = \sum F(g_vec[rho])
4. call build_Q_polynomial
5. return Q_poly, fj_poly, instance

Verifier:
1. pick random index rho
2. evaluate Ti from fj polys
3. check if Ti = claim by sumcheck protocol
4. apply sumcheck protocol
5. commit to fj polys
6. return Ti, commit(f1), commit(f2), ..., commit(ft)

For simplicity, verifier returns true/false instead of committing fj polys.

## Reference

1. [KS24](https://eprint.iacr.org/2024/1606)
2. [GKR15](https://dl.acm.org/doi/10.1145/2699436)
3. [XZZ19](https://eprint.iacr.org/2019/317.pdf)