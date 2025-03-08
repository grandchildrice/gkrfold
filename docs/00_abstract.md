## Abstract

In this paper, we introduce GKRFold, a protocol for compressing the multiple GKR-based SNARK proofs into a single proof.

GKRFold incorporates SumFold—a component originally proposed in NeutronNova[KS24]—to achieve this compression. Ultimately, the compression process yields a single GKR or Sumcheck instance along with one commitment.

As a result, the verifier is required to perform only one Sumcheck and two random polynomial evaluations. Moreover, the protocol is applicable to both uniform and non-uniform instances of the GKR protocol.
