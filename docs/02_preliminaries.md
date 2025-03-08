## 2. Preliminaries

### 2-1. GKR Protocol

Its design is based on representing each circuit layer as a corresponding layer polynomial. Starting from the output, the protocol sequentially relays a claim about one layer to a claim about the subsequent layer by executing a Sumcheck protocol on the evaluation of the layer polynomial at a randomly chosen point.

![image](https://hackmd.io/_uploads/rJDhMAMc1l.png)

More precisely, the layer polynomial $ V_i $ is defined as
$$
V_i(z) = \sum_{x,y \in \{0,1\}^{S_i}} \Bigl\{ add_{i+1}(z,x,y)\bigl(V_{i+1}(x)+V_{i+1}(y)\bigr) + mult_{i+1}(z,x,y)\bigl(V_{i+1}(x) \cdot V_{i+1}(y)\bigr) \Bigr\},
$$
where the functions $add_i(z,x,y)$ and $mult_i(z,x,y)$ return 1 if the gate $(z,x,y)$ in layer $i$ is an addition or multiplication gate, respectively, and 0 otherwise.

An overview of the GKR protocol is as follows:
1. The prover sends the circuit evaluation $C(x,w)=y$ to the verifier.
2. For every layer $i = 1,\dots,d-1$ (excluding the input layer), the verifier issues a challenge by selecting a random point $r_i$ and then initiates the Sumcheck protocol on $V_i(r_i)$.
3. The verifier directly evaluates the input layer $V_d(r_d)$.


### 2-2. Linear GKR Protocol (Libra[XZZ19])

Libra[XZZ19] achieves the linear-time prover algorithm for the GKR protocol by applying 2-phases Sumcheck.

Let's assume that the following layer polynomial is given:

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

1. Receive a random value $r_i$ from V
2. Prove the correctness of $h_{i+1}(r_i)$ via Sumcheck.
3. Receive a random value $u_i$ from V
4. Prove the correctness of the term $\sum_{x \in \{0,1\}^{S_i}} h_{i+1}(x)V_{i+1}(u_i,x)$ via Sumcheck.

Thus, for each layer, a total of $2 \times 3 = 6$ Sumcheck instances are executed in parallel.