Below is a polished, paper-style version in English, with all mathematical expressions enclosed in \$ or \$\$ as requested.

---

## 4. GKRFold

GKRFold is a technique that leverages SumFold to compress $ n $ GKR instances into a single instance. In what follows, we describe two variants: the normal GKRFold (GKR-to-GKR) and the ultra GKRFold (GKR-to-Sumcheck).

---

### 4-1. Normal GKRFold: GKR-to-GKR

In the normal GKRFold, the idea is to directly apply the SumFold method to GKR instances. The method proceeds as follows:

1. **Function Construction and Commitment:**
   The prover constructs functions $ f_{j}(b,x) $ and commits to them with the verifier. Here, $ f_{j}(i, x) $ is defined to return the layer polynomial corresponding to layer $ j $ of the $ i $-th GKR instance. In effect, the polynomial $ f_{j}(b,x) $ is designed so that when $ b = i $ its output equals the polynomial of the $ i $-th instance.

2. **Random Challenge and Instance Selection:**
   In response to the verifier’s random challenge $ \rho $, the GKR protocol is executed on the $ \rho $-th GKR instance. The randomness ensures that the prover cannot selectively cheat on a subset of instances.

3. **Output:**
   The $ \rho $-th GKR instance is then output as the folded instance. The correctness of the folding is ensured by the underlying SumFold protocol.

---

### 4-2. Ultra GKRFold: GKR-to-Sumcheck

Next, we describe a method to fold $ n $ GKR instances into a single Sumcheck instance. In this variant, SumFold is applied to the layer polynomials of the GKR instances, effectively reducing the Sumcheck instances corresponding to a given layer. Recall that, as described in Section 2-2, the GKR layer Sumcheck splits into a 3-term, 2-phase structure. With suitable mappings, the following table summarizes the correspondence:

|                        | **Sumcheck 1** $ (h_{1,i+1}(x)) $         | **Sumcheck 2** $ (h_{2,i+1}(y)) $         | **Sumcheck 3** $ (h_{3,i+1}(x)) $         | **Sumcheck 4** (1st term)                      | **Sumcheck 5** (2nd term)                      | **Sumcheck 6** (3rd term)                      |
|------------------------|---------------------------------------------|---------------------------------------------|---------------------------------------------|------------------------------------------------|------------------------------------------------|------------------------------------------------|
| **Expression**         | $\displaystyle \sum_{y \in \{0,1\}^{S_i}} add_{i+1}(z,x,y)$ | $\displaystyle \sum_{x \in \{0,1\}^{S_i}} add_{i+1}(z,x,y)$ | $\displaystyle \sum_{y \in \{0,1\}^{S_i}} mult_{i+1}(z,x,y)V_{i+1}(y)$ | $\displaystyle \sum_{x \in \{0,1\}^{S_i}} h_{1,i+1}(x)V_{i+1}(x)$ | $\displaystyle \sum_{y \in \{0,1\}^{S_i}} h_{2,i+1}(y)V_{i+1}(y)$ | $\displaystyle \sum_{x \in \{0,1\}^{S_i}} h_{3,i+1}(x)V_{i+1}(x)$ |
| **Form of** $F$       | $F(g_1(x),g_2(x)) = g_1(x) \cdot g_2(x)$  | same                                        | same                                        | same                                           | same                                           | same                                           |
| **Vector** $\vec{g}$  | $\{ add_{i+1}(z,x,y),\, g_e(x,y,z) \}$      | $\{ add_{i+1}(z,x,y),\, g_e(x,y,z) \}$      | $\{ mult_{i+1}(z,x,y),\, V_{i+1}(y) \}$     | $\{ h_{1,i+1}(x),\, V_{i+1}(x) \}$            | $\{ h_{2,i+1}(y),\, V_{i+1}(y) \}$            | $\{ h_{3,i+1}(x),\, V_{i+1}(x) \}$            |

Here, $ g_e(x) = 1 $ is the identity function that always returns 1.

Note that Sumcheck 1 and 4, Sumcheck 2 and 5, Sumcheck 3 and 6 correspond respectively to phase 1 and phase 2 of the GKR protocol.

Using this mapping, the $ n \times d $ Sumcheck instances can be arranged as indicated in the figure below. (Each instance $ T $ at this stage remains to be determined.)

<img src="https://hackmd.io/_uploads/HJo8TwWqkg.png" width="400" />

Based on the above mapping, GKRFold functions as follows.

---

#### Step 1. Sumcheck Initialization (for all layers $ 1,\dots,d $)

The verifier sends random challenges $ u_1, u_2, \dots, u_{3n} $ to the prover. Then, based on these random values, the prover constructs $ 3*n $ Sumcheck instances corresponding to the entries in columns 1, 2, and 3 of the table above and sends them to the verifier.

Next, without running the Sumcheck immediately, the verifier sends random challenges $ r_1, r_2, \dots, r_{3n} $ to the prover.

Subsequently, the prover constructs another set of $ 3*n $ Sumcheck instances corresponding to the entries in columns 4, 5, and 6 (again, based on the random challenges) and sends them to the verifier.

Repeating this process for layers 1 through $ d $ initializes a total of $ 6 \times n \times d $ Sumcheck instances.

---

#### Step 2. Commitment Construction by the Prover

The prover constructs the polynomials
$$
f_1(b,x),\quad f_2(b,x)
$$
and sends a commitment to these functions. Each polynomial is defined such that its output corresponds to $ g_{bj}(x) $ for the $ b $-th instance, ensuring that when $ b $ is fixed, the correct layer polynomial is recovered.

<img src="https://hackmd.io/_uploads/HJo8TwWqkg.png" width="400" />

---

#### Step 3. Running SumFold

Finally, the SumFold protocol is executed in a manner analogous to the standard SumFold. Multiple rounds of challenges are issued. Depending on the outcome of these challenges, indices corresponding to the phases (phase 1 and phase 2) are selected, and the GKR Round Sumcheck is executed accordingly.

### 4-3. Cost Analysis

TODO...

### 4-4. Proof of Theorem (Informal)

TODO...
