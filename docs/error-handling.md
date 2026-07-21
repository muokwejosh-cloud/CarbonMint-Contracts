# Error Handling

This note documents the **error-handling** of the carbonmint-contract contract.

carbonmint-contract is a Soroban smart contract on the Stellar network. This page is part of the
project's reference documentation and describes the error-handling in detail, covering the relevant
entrypoints, storage layout, and invariants where applicable.

Checked arithmetic helpers in the contract return [`crate::Error`] values rather than panicking. Callers
should propagate those errors with `?` and handle them explicitly at the boundary. For example, a checked
addition should be written as:

```rust
use carbonmint_contract::{math, Error};

fn add_checked(a: i128, b: i128) -> Result<i128, Error> {
    math::checked_add(a, b)
}
```

See the README and the sources under src/ for the authoritative implementation.
