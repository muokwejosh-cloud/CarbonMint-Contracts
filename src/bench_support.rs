//! Shared helpers for benchmark-oriented contract setup.
//!
//! The module mirrors the regular test harness so the benchmark can reuse the
//! same contract registration pattern without diverging from the main test
//! setup.

#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{CarbonMintContract, CarbonMintContractClient};

pub(crate) fn setup_contract<'a>() -> (Env, CarbonMintContractClient<'a>, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, CarbonMintContract);
    let client = CarbonMintContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    (env, client, admin)
}
