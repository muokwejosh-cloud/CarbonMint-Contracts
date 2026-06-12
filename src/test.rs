#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

use crate::{CarbonMintContract, CarbonMintContractClient};

/// Registers the contract and returns its client together with the env.
fn setup<'a>() -> (Env, CarbonMintContractClient<'a>, Address) {
    let env = Env::default();
    let contract_id = env.register(CarbonMintContract, ());
    let client = CarbonMintContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    (env, client, admin)
}

/// Convenience helper to build a project id string in tests.
fn project_id(env: &Env) -> String {
    String::from_str(env, "PROJ-001")
}
