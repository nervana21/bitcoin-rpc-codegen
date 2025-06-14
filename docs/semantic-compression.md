# Semantic Compression and the Generative Complexity of Protocol Interfaces

This document formalizes the theoretical foundation behind [`bitcoin-rpc-codegen`](https://github.com/nervana21/bitcoin-rpc-codegen). It introduces **semantic compression**, a framework for evaluating how compactly a generator can produce correct implementations of a protocol interface based on its semantic specification.

## Core Idea

Inspired by **Kolmogorov Complexity**—which measures the length of the shortest program that outputs a given string—**semantic compression** asks:

> _What is the smallest generator that produces implementations satisfying a given semantic specification?_

The smaller the generator, the more efficiently it expresses the interface's semantics.

We define the **Generative Complexity** of a protocol interface $\mathcal{S}$ as the size of the smallest generator $\Gamma$ that, given a structured description $\Delta$ of its semantic specification, produces a correct implementation $\mathcal{I}$:

```math
\text{GC}(\mathcal{S}) = \min_{\Gamma : \Gamma(\Delta) \to \mathcal{I} \models \mathcal{S}} \; \|\Gamma\|
```

Where:

- $|\Gamma|$ is the size of the generator (measured in tokens, bytes, or AST nodes)
- $\mathcal{I} \models \mathcal{S}$ means the implementation conforms to the semantic specification

## Beyond One Version: Interface Families

The approach extends naturally to **versioned families** of protocol interfaces. Rather than targeting a single $\mathcal{S}$, the generator supports a set $\{\mathcal{S}_v\}$ corresponding to successive Bitcoin Core versions:

- $\{\Delta_v\}$ = Structured descriptions for each version (e.g., `api_v27.json`, `api_v28.json`, …)
- $\Gamma(\Delta_v) = \mathcal{I}_v$, such that $\mathcal{I}_v \models \mathcal{S}_v$ for all $v$

```math
\text{GC}(\{\mathcal{S}_v\}) = \min_{\Gamma : \forall v, \Gamma(\Delta_v) \to \mathcal{I}_v \models \mathcal{S}_v} \; \|\Gamma\|
```

The generator now compresses not just one specification, but an **evolving interface family**.

The approach scales:

- From a single interface $\mathcal{S}$
- To versioned families $\{\mathcal{S}_v\}$
- To additional structured components of Bitcoin Core, including indexing, wallet logic, and mempool policy

A future generator $\Gamma$ capable of spanning these domains would enable much of Bitcoin Core's non-[consensus-critical](https://github.com/rust-bitcoin/rust-bitcoin?tab=readme-ov-file#consensus) logic to be described, reviewed, and reused with confidence. While consensus code must remain implementation-defined, structured modeling may still support its safety and evolution.

## Application to Bitcoin Core RPC

This project applies semantic compression to the Bitcoin Core RPC interface:

- `S` = The semantic specification of Bitcoin Core RPC version 28
- `Δ` = A structured schema describing `S` (e.g., `api_v28.json`)
- `Γ` = [`bitcoin-rpc-codegen`](https://github.com/nervana21/bitcoin-rpc-codegen)
- `I` = [`bitcoin-rpc-midas`](https://github.com/nervana21/bitcoin-rpc-midas`), the generated Rust client

The generator `Γ` reads `Δ` and emits a complete implementation `I`—a type-safe client library that satisfies the specification `S`. The goal is to minimize the size of `Γ` while ensuring that `I` correctly implements `S`.

This mirrors the architectural goals of the [Bitcoin Core multiprocess project](https://github.com/bitcoin/bitcoin/pull/28722), which uses `.capnp` files and the `mpgen` tool to define and generate C++ interfaces between components like `bitcoin-node` and `bitcoin-wallet`. In this project, `api.json` plays the same role: it is a structured, machine-generated schema that captures the behavior and structure of the RPC interface used to generate Rust clients.

There is, both in principle and in practice, a one-to-one mapping between `.capnp` and `api.json`. Both formats serve as formal interface descriptions. If the multiprocess project formalizes `.capnp` as the canonical source of truth, this project will adopt it directly and derive `api.json` from it. Until then, `api.json` may serve as a reference description—a practical and extensible foundation that other tools and implementations can build on.

## Compression Ratio

To quantify efficiency, we define the **Semantic Compression Ratio (SCR)**:

- For a single specification:

```math
\text{SCR}(\mathcal{S}, \Gamma) = \frac{|\mathcal{I}|}{\|\Gamma\|}
```

- For a versioned set:

```math
\text{SCR}(\{\mathcal{S}_v\}, \Gamma) = \frac{\sum_v |\mathcal{I}_v|}{\|\Gamma\|}
```

This gives a measure of **semantic density**—the total size of all generated implementations, divided by the size of the generator. It reflects how much correct behavior is produced across all supported versions per unit of generator logic.

## Practical Implications

- A smaller $|\Gamma|$ means the interface is being captured more compactly
- Supporting more $\{\mathcal{S}_v\}$ without increasing $|\Gamma|$ shows better generalization
- Comparing different generators for the same $\mathcal{S}$ gives a principled way to evaluate design efficiency

Together, these tools support maintainability and long-term evolution without code duplication.

Here’s a cleaner, more compact version of your summary, with improved flow and rhythm:

---

## Summary

Semantic compression reframes code generation as the search for the smallest generator that produces correct, version-aware implementations of a protocol. In `bitcoin-rpc-codegen`, this idea becomes practical: a way to simplify and stabilize the developer surface without losing precision.

By capturing Bitcoin Core’s RPC interface in a compact, generative form, the project:

- Makes behavior easier to verify and harder to misuse
- Tracks changes across versions systematically
- Produces consistent, type-safe clients with minimal duplication

This foundation guides the project’s design, extensions, and tooling. The goal is to reduce the operational complexity of working with Bitcoin—making the protocol more accessible, stable, and easier to extend.
