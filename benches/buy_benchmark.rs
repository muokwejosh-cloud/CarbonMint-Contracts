use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

use carbonmint_contract::{CarbonMintContract, CarbonMintContractClient};

fn buy_entrypoint_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("carbonmint-buy");
    group.bench_function("buy", |b| {
        b.iter_batched(
            || {
                let env = Env::default();
                let contract_id = env.register_contract(None, CarbonMintContract);
                let client = CarbonMintContractClient::new(&env, &contract_id);

                let admin = Address::generate(&env);
                let issuer = Address::generate(&env);
                let buyer = Address::generate(&env);
                let project_id = String::from_str(&env, "BENCH-001");

                env.mock_all_auths();
                client.initialize(&admin);
                let batch_id =
                    client.mint_batch(&issuer, &project_id, &2024u32, &10_000i128, &5i128);

                (env, client, buyer, batch_id)
            },
            |(env, client, buyer, batch_id)| {
                let _ = client.buy(&buyer, &batch_id, &100i128);
                let _ = &env;
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, buy_entrypoint_benchmark);
criterion_main!(benches);
