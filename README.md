# CarbonMint

A tokenized **carbon-credit marketplace** smart contract for the
[Stellar](https://stellar.org) Soroban platform, written in Rust with
`soroban-sdk` 21.x.

CarbonMint lets issuers register batches of carbon credits, list them for sale,
sell them to buyers, and lets holders permanently **retire** credits in exchange
for an on-chain retirement certificate.

## Model

Credits are **semi-fungible**: every credit within the same batch is
interchangeable, but credits from different batches are tracked separately.
Balances are keyed by `(owner, batch_id)`.

- **Batch** — a registered lot of credits with an issuer, project id, vintage,
  original supply, listed price, and a listing flag.
- **Retirement** — a certificate recording the permanent burning of a quantity
  of credits by a holder.

## Contract API

### State-changing

| Function | Description |
| --- | --- |
| `initialize(admin)` | Sets the registry admin. Callable once. |
| `mint_batch(issuer, project_id, vintage, amount, price) -> u64` | Registers a batch and credits the issuer; returns the new batch id. Requires issuer auth. |
| `list(batch_id, price)` | Lists / reprices a batch. Requires issuer auth. |
| `buy(buyer, batch_id, amount)` | Transfers credits from the seller to the buyer (mock payment). Requires buyer auth. |
| `retire(holder, batch_id, amount) -> u64` | Burns credits and issues a retirement certificate; returns the certificate id. Requires holder auth. |

### Read-only

| Function | Description |
| --- | --- |
| `get_admin() -> Address` | The registry admin. |
| `balance_of(owner, batch_id) -> i128` | Credit balance of an owner for a batch. |
| `get_batch(batch_id) -> Batch` | The batch record. |
| `get_retirement(cert_id) -> Retirement` | A retirement certificate. |
| `total_retired(batch_id) -> i128` | Total credits retired for a batch. |
| `batch_count() -> u64` | Number of batches minted. |
| `retirement_count() -> u64` | Number of certificates issued. |

## Errors

| Code | Variant | Meaning |
| --- | --- | --- |
| 1 | `AlreadyInitialized` | `initialize` called more than once. |
| 2 | `NotInitialized` | Contract used before `initialize`. |
| 3 | `BatchNotFound` | Unknown batch or certificate id. |
| 4 | `InvalidAmount` | Amount is zero / negative or price is negative. |
| 5 | `InsufficientBalance` | Holder lacks enough credits. |
| 6 | `Unauthorized` | Caller not permitted. |
| 7 | `Overflow` | Arithmetic overflow. |
