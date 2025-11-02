> **Note:** This document is part of the archived `bitcoin-rpc-codegen` repository. The concepts have been evolved and updated in the [Ethos](https://github.com/nervana21/ethos) project. For the latest version, see [semantic-convergence.md](https://github.com/nervana21/ethos/blob/main/docs/semantic-convergence.md) in the Ethos repository.

# Semantic Compression and Protocol Interface Complexity

This document explains the theory behind [`bitcoin-rpc-codegen`](https://github.com/nervana21/bitcoin-rpc-codegen), introducing **semantic compression**: a way to measure how compactly a generator can produce correct protocol implementations from a semantic specification.

## Core Idea

Inspired by **Kolmogorov Complexity**—the length of the shortest program that outputs a string—**semantic compression** asks:

> _What is the smallest generator that produces implementations matching a given specification?_

A smaller generator means a more efficient expression of the interface.

We define the **Generative Complexity** of a protocol interface $\mathcal{S}$ as the size of the smallest generator $\Gamma$ that, given a structured description $\Delta$, produces a correct implementation $\mathcal{I}$:

```math
\text{GC}(\mathcal{S}) = \min_{\Gamma : \Gamma(\Delta) \to \mathcal{I} \models \mathcal{S}} \; \|\Gamma\|
```

Where:

- $\|\Gamma\|$ = size of the generator (tokens, bytes, or AST nodes)
- $\mathcal{I} \models \mathcal{S}$ = implementation matches the specification

## Interface Families

This approach extends to **versioned families** of interfaces. Instead of one $\mathcal{S}$, the generator supports a set $\{\mathcal{S}_v\}$ for different Bitcoin Core versions:

- $\{\Delta_v\}$ = Structured descriptions for each version (e.g., `bitcoin-core-api.json` with embedded version)
- $\Gamma(\Delta_v) = \mathcal{I}_v$, with $\mathcal{I}_v \models \mathcal{S}_v$ for all $v$

```math
\text{GC}(\{\mathcal{S}_v\}) = \min_{\Gamma : \forall v, \Gamma(\Delta_v) \to \mathcal{I}_v \models \mathcal{S}_v} \; \|\Gamma\|
```

The generator now compresses an **evolving interface family**.

This scales from a single interface, to versioned families, to other structured Bitcoin Core components (indexing, wallet, mempool policy). A future generator could span much of Bitcoin Core's non-[consensus-critical](https://github.com/rust-bitcoin/rust-bitcoin?tab=readme-ov-file#consensus) logic. Consensus code must remain implementation-defined, but structured modeling can still help its safety and evolution.

## Bitcoin Core RPC Example

In this project:

- `S` = Bitcoin Core RPC v28 specification
- `Δ` = Structured schema (e.g., `bitcoin-core-api.json`)
- `Γ` = [`bitcoin-rpc-codegen`](https://github.com/nervana21/bitcoin-rpc-codegen)
- `I` = [`bitcoin-rpc-midas`](https://github.com/nervana21/bitcoin-rpc-midas), the generated Rust client

The generator `Γ` reads `Δ` and emits a type-safe client `I` that matches `S`. The goal: minimize `Γ`'s size while ensuring `I` is correct.

This mirrors the [Bitcoin Core multiprocess project](https://github.com/bitcoin/bitcoin/pull/28722), which uses `.capnp` files and `mpgen` to generate C++ interfaces. Here, `api.json` plays the same role: a machine-generated schema for generating Rust clients.

There is a one-to-one mapping between `.capnp` and `api.json`. If `.capnp` becomes canonical, this project will adopt it and derive `api.json` from it. Until then, `api.json` is a practical, extensible reference.

## Compression Ratio

To measure efficiency, define the **Semantic Compression Ratio (SCR)**:

- For a single specification:

```math
\text{SCR}(\mathcal{S}, \Gamma) = \frac{|\mathcal{I}|}{\|\Gamma\|}
```

- For a versioned set:

```math
\text{SCR}(\{\mathcal{S}_v\}, \Gamma) = \frac{\sum_v |\mathcal{I}_v|}{\|\Gamma\|}
```

This measures **semantic density**: total size of all generated implementations, divided by generator size. It shows how much correct behavior is produced per unit of generator logic.

## Practical Implications

- Smaller $\|\Gamma\|$ = more compact interface capture
- Supporting more $\{\mathcal{S}_v\}$ without growing $\|\Gamma\|$ = better generalization
- Comparing generators for the same $\mathcal{S}$ = principled design efficiency

These tools support maintainability and long-term evolution without code duplication.

## Summary

Semantic compression reframes code generation as finding the smallest generator for correct, version-aware protocol implementations. In `bitcoin-rpc-codegen`, this means:

- Easier verification and safer behavior
- Systematic tracking of version changes
- Consistent, type-safe clients with minimal duplication

This foundation guides the project's design and tooling, aiming to make Bitcoin more accessible, stable, and extensible.
