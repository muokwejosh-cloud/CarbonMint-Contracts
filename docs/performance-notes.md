# Performance Notes

This note documents the **performance-notes** of the carbonmint-contract contract.

carbonmint-contract is a Soroban smart contract on the Stellar network. This page is part of the
project's reference documentation and describes the performance-notes in detail, covering the relevant
entrypoints, storage layout, and invariants where applicable.

A lightweight Criterion benchmark for the hottest entrypoint, `buy`, is available at `benches/buy_benchmark.rs`.
It exercises the in-process Soroban environment and measures the contract call path for a simple buy flow.

Run it with:

```sh
make bench
```

See the README and the sources under src/ for the authoritative implementation.
