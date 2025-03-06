
## Abstract

In this paper, we introduce GKRFold, a protocol for compressing both the proof size and the verification cost of *n* GKR-based SNARK proofs. GKRFold incorporates SumFold—a component originally proposed in NeutronNova[KS24]—to achieve this compression. Ultimately, the compression process yields a single Sumcheck instance along with one commitment. As a result, the verifier is required to perform only one Sumcheck and two random polynomial evaluations. Moreover, the protocol is applicable to both uniform and non-uniform instances of the GKR protocol.

## 1. Introduction

### 1.1. GKR Protocol

The GKR proof system[GKR15,XZZ19], a prominent SNARK (Succinct Non-interactive ARgument of Knowledge) due to its efficient prover time, has been employed in systems such as [Expander](https://github.com/PolyhedraZK/Expander) and [Ceno](https://github.com/scroll-tech/ceno). Its design is based on representing each circuit layer as a corresponding layer polynomial. Starting from the output, the protocol sequentially relays a claim about one layer to a claim about the subsequent layer by executing a Sumcheck protocol on the evaluation of the layer polynomial at a randomly chosen point.

![image](https://hackmd.io/_uploads/rJDhMAMc1l.png)

More precisely, the layer polynomial \( V_i \) is defined as
$$
V_i(z) = \sum_{x,y \in \{0,1\}^{S_i}} \Bigl\{ add_{i+1}(z,x,y)\bigl(V_{i+1}(x)+V_{i+1}(y)\bigr) + mult_{i+1}(z,x,y)\bigl(V_{i+1}(x) \cdot V_{i+1}(y)\bigr) \Bigr\},
$$
where the functions $add_i(z,x,y)$ and $mult_i(z,x,y)$ return 1 if the gate $(z,x,y)$ in layer $i$ is an addition or multiplication gate, respectively, and 0 otherwise.

An overview of the GKR protocol is as follows:
1. The prover sends the circuit evaluation $C(x,w)=y$ to the verifier.
2. For every layer $i = 1,\dots,d-1$ (excluding the input layer), the verifier issues a challenge by selecting a random point $r_i$ and then initiates the Sumcheck protocol on $V_i(r_i)$.
3. The verifier directly evaluates the input layer $V_d(r_d)$.

Although the prover can generate proofs in $O(N)$ time without committing to intermediate results or relying on FFTs, the verification time and proof size exhibit a complexity of $O(D\log N)$. Consequently, achieving efficient proof compression remains an important challenge.

### 1.2. NeutronNova and SumFold

SumFold, as introduced in NeutronNova[KS24], is a technique that folds an arbitrary number of Sumcheck instances into a single instance. By applying SumFold to the GKR protocol, it is possible to compress the multiple Sumcheck instances inherent in GKR proofs, thereby reducing both the overall proof size and the verification time.

### 1.3. Contributions

In this work, we propose GKRFold, a novel method that applies SumFold to efficiently compress $n$ instances of GKR proofs. The primary contributions of this study are as follows:
1. We demonstrate that SumFold can be effectively utilized to compress GKR instances, leading to substantial improvements in proof size and verification efficiency.
2. We show that GKRFold can compress $n$ GKR proofs, each comprising $d$ layers (amounting to a total of $6 \times n \times d$ Sumcheck instances), into a single Sumcheck instance.

This result significantly reduces the overhead associated with verifying multiple GKR proofs, making the protocol more practical for applications requiring succinct and efficient verification.

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

Protocol (cited from [KS24]):
![Screenshot from 2025-02-16 11-15-57](https://hackmd.io/_uploads/S1icthJq1x.png)

## 3. GKRFold

### 3.0. Linear GKR Protocol (Libra[XZZ19])

In this section, we describe the linear-time algorithm for the GKR protocol. First, assume that the following layer polynomial is given:

$$
V_i(z) = \sum_{x,y \in \{0,1\}^{S_i}} \Bigl\{ add_{i+1}(z,x,y)\bigl(V_{i+1}(x)+V_{i+1}(y)\bigr) + mult_{i+1}(z,x,y)\bigl(V_{i+1}(x) \cdot V_{i+1}(y)\bigr) \Bigr\}.
$$

We partition this expression into three terms:

$$
V_i(z) = \sum_{x,y \in \{0,1\}^{S_i}} add_{i+1}(z,x,y)V_{i+1}(x) \;+\;
$$

$$
\sum_{x,y \in \{0,1\}^{S_i}} add_{i+1}(z,x,y)V_{i+1}(y) \;+\;
$$

$$
\sum_{x,y \in \{0,1\}^{S_i}} mult_{i+1}(z,x,y)\bigl(V_{i+1}(x) \cdot V_{i+1}(y)\bigr).
$$

For each term, a 2-phase Sumcheck protocol is executed. For instance, consider the third term:

$$
\sum_{x,y \in \{0,1\}^{S_i}} mult_{i+1}(z,x,y)\bigl(V_{i+1}(x) \cdot V_{i+1}(y)\bigr)
=\sum_{x \in \{0,1\}^{S_i}} h_{i+1}(x)V_{i+1}(x),
$$

where

$$
h_{i+1}(x) = \sum_{y \in \{0,1\}^{S_i}} mult_{i+1}(z,x,y)V_{i+1}(y).
$$

The 2-phase Sumcheck protocol is then carried out as follows:

1. Prove the correctness of $h_{i+1}(x)$ via Sumcheck.
2. Prove the correctness of the term $\sum_{x \in \{0,1\}^{S_i}} h_{i+1}(x)V_{i+1}(x)$ via Sumcheck.

Thus, for each layer, a total of $2 \times 3 = 6$ Sumcheck instances are executed in parallel.

### 3.1. GKRFold: Overview

Assume that we are given $n$ GKR instances, and that each instance comprises $d$ layers. Then there exist a total of $n \times d$ layer polynomials. For each layer, the protocol requires 6 Sumcheck instances (as described above), resulting in a total of

$$
6 \times n \times d
$$

Sumcheck instances.

These six types of Sumcheck instances are summarized in the following table:

|                        | **Sumcheck 1** $(h_{1,i+1}(x))$         | **Sumcheck 2** $(h_{2,i+1}(y))$         | **Sumcheck 3** $(h_{3,i+1}(x))$         | **Sumcheck 4** (1st term)                      | **Sumcheck 5** (2nd term)                      | **Sumcheck 6** (3rd term)                      |
|------------------------|-----------------------------------------|-----------------------------------------|-----------------------------------------|-----------------------------------------------|-----------------------------------------------|-----------------------------------------------|
| **Expression**         | $\sum_{y \in \{0,1\}^{S_i}} add_{i+1}(z,x,y)$ | $\sum_{x \in \{0,1\}^{S_i}} add_{i+1}(z,x,y)$ | $\sum_{y \in \{0,1\}^{S_i}} mult_{i+1}(z,x,y)V_{i+1}(y)$ | $\sum_{x \in \{0,1\}^{S_i}} h_{1,i+1}(x)V_{i+1}(x)$ | $\sum_{y \in \{0,1\}^{S_i}} h_{2,i+1}(y)V_{i+1}(y)$ | $\sum_{x \in \{0,1\}^{S_i}} h_{3,i+1}(x)V_{i+1}(x)$ |
| **Form of** $F$       | $F(g_1(x),g_2(x)) = g_1(x) \cdot g_2(x)$  | same                                    | same                                    | same                                          | same                                          | same                                          |
| **Vector** $\vec{g}$  | $\{ add_{i+1}(z,x,y),\, g_e(x,y,z) \}$      | $\{ add_{i+1}(z,x,y),\, g_e(x,y,z) \}$      | $\{ mult_{i+1}(z,x,y),\, V_{i+1}(y) \}$      | $\{ h_{1,i+1}(x),\, V_{i+1}(x) \}$             | $\{ h_{2,i+1}(y),\, V_{i+1}(y) \}$             | $\{ h_{3,i+1}(x),\, V_{i+1}(x) \}$             |

Note that Sumcheck 1 and Sumcheck 2 naturally have the form

$$
F(g_1(x)) = g_1(x).
$$

However, by introducing the constant identity function $g_e(x) = 1$ (which always returns 1), they can equivalently be expressed as

$$
F(g_1(x), g_e(x)) = g_1(x) \cdot g_e(x).
$$

Thus, all Sumcheck instances share the same functional form

$$
F(g_1(x), g_2(x)) = g_1(x) \cdot g_2(x).
$$

The correspondence between these Sumcheck instances and the SumFold framework is given in the table above.

Consequently, by applying SumFold, the total of

$$
6 \times n \times d
$$

Sumcheck instances can be folded into a single Sumcheck instance.

<img src="https://hackmd.io/_uploads/HJo8TwWqkg.png" width="400" />

### 3.2. GKRFold: Protocol

**Input:**
$n$ GKR instances, represented as

$$
\Bigl\{\{SC_{i1}, SC_{i2}, SC_{i3}, SC_{i4}, SC_{i5}, SC_{i6}\}_{i \in [n \times d]},\, (\vec{x}_i, \vec{w}_i)_{i \in [n \times d]},\, (\vec{u}_i)_{i \in [n \times d]}\Bigr\} \in GKR^{n},
$$

**Output:**
$(SC,\, r_b,\, \bar{Q}(r_b),(\vec{x}_i, \vec{w}_i), (\vec{u}_i))$

**Protocol:**

1. The prover sends the evaluation results $C_i(x,w) = y$ for each instance to the verifier.
2. Based on the layer polynomials $V_i$, assign the six Sumcheck instances (SC1 through SC6) for each layer.
3. Execute SumFold under the following conditions:
   - The function is defined as

     $$
     F(g_1(x), g_2(x)) = g_1(x) \cdot g_2(x).
     $$

   - The vector $\vec{g}$ is instantiated as follows:
     - **SC$_1$:** $\{ add_{i+1}(z,x,y),\, g_e(x,y,z) \}$
     - **SC$_2$:** $\{ add_{i+1}(z,x,y),\, g_e(x,y,z) \}$
     - **SC$_3$:** $\{ mult_{i+1}(z,x,y),\, V_{i+1}(y) \}$
     - **SC$_4$:** $\{ h_{1,i+1}(x),\, V_{i+1}(x) \}$
     - **SC$_5$:** $\{ h_{2,i+1}(y),\, V_{i+1}(y) \}$
     - **SC$_6$:** $\{ h_{3,i+1}(x),\, V_{i+1}(x) \}$
4. Verify that for all $i = 1, \dots, n$,

   $$
   Q(6d \cdot i) = v_{i,d}(r_d),
   $$

   where $Q$ is the polynomial constructed via SumFold and $v_{i,d}(r_d)$ denotes the evaluation of the corresponding layer polynomial at the challenge $r_d$.
5. The verifier evaluates the input layer $V_d(r_d)$.