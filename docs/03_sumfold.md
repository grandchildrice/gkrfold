## 3. SumFold

SumFold, as introduced in NeutronNova[KS24], is a technique that folds an arbitrary number of Sumcheck instances into a single instance.

### 3-1. Protocol Overview

Assume that we want to prove, via Sumcheck, a function defined by a composition $T = F(\vec{g}(x))$.

The goal of SumFold is to reduce the $n$ Sumcheck instances
to a single instance.

![image](https://hackmd.io/_uploads/HyGj7mWc1g.png)

#### Step 1: Commitment Construction by the Prover

The prover constructs the polynomials
$$
f_1(b,x),\ f_2(b,x),\ \dots,\ f_t(b,x)
$$

and sends a commitment to these functions. Each polynomial is defined such that its output corresponds to $g_{bj}(x)$ for the $b$-th instance.

![Figure: Commitment Diagram](https://hackmd.io/_uploads/BkyoQGGcJe.png)

#### Step 2: Random Challenge by the Verifier

The verifier selects a random field element $\rho$ and transmits it to the prover.

#### Step 3: Construction of $Q(b)$ by the Prover

The prover constructs the polynomial $Q(b)$ such that:
$$
Q(\rho) = T_{\rho}, \quad \text{and} \quad Q(b) = 0 \text{ for } b \neq \rho.
$$

#### Step 4: Execution of the Sumcheck Protocol

Both the prover and the verifier engage in the Sumcheck protocol. Initially, the verifier checks that
$$
T' = Q(\rho).
$$

Subsequently, the protocol proves the following equality:
$$
T' = \sum_{x} F\bigl(f_1(\rho,x),\, f_2(\rho,x),\, \dots,\, f_t(\rho,x)\bigr)
$$
using the Sumcheck protocol. Notably, Steps 2 and 3 are iterated for at most $\log(n)$ rounds.

#### Step 5: Output Generation

Upon the completion of the above steps, the protocol outputs the folded instance.

---

### 3-2. Proof of Theorem (Informal)

Completeness:

The folded instance produced by SumFold is correctly contained within the Sumcheck instance. Under the assumption that each individual Sumcheck instance is valid, the instance $T'$ obtained through the folding procedure by the prover and verifier corresponds to a randomly chosen instance among $n$ instances.

Specifically, if the prover correctly prepares the value $ T_i$, then for any selected instance $i$ it holds that
$$
Q(i) = T_i.
$$
Furthermore, the sum of the polynomial $ F\bigl(\vec{g}_i(x)\bigr) $ verified by the Sumcheck protocol is exactly equal to $ T_i $.

Knowledge Soundness:

Due to the commitment scheme, the prover is unable to conveniently modify the instance after learning the random challenge.

Moreover, if a dishonest prover were to embed an invalid instance among the $n$ Sumcheck instances in advance, and the verifier subsequently selects that instance, the soundness of the Sumcheck protocol ensures that it will be rejected with high probability.

In the event that the verifier does not select the invalid instance, the instance might pass verification; however, by iterating the challenge process multiple times, the verifier can ultimately achieve a very high probability of rejecting such a dishonest attempt.

### 3-3. Costs

Let
- $t$ denotes the element size of $\vec{g}$.
- $D$ denotes the degree of $F$,
- $n$ denotes the number of instances
- $l$ denotes the number of variables in $g$.

The complexities are summarized in the following table:

| **Round**         | **Communication**         | **Prover Time**                         | **Verifier Time**      |
|-------------------|----------------------------------------|----------------------------------------------------|-------------------------------------|
| $1+\log(n)$      | $O(D \log(n))$ field elements         | $O(n \cdot t \cdot D \cdot 2^l)$ field elements            | $O(D \log(n))$ field elements      |

By following Theorem 2 in the NeutronNova paper[KS24].