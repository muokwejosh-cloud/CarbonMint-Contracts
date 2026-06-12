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
