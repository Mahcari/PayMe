#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Map};

/// Initialises a fresh contract and returns (env, client, admin, signer_map).
fn setup_initialized_contract(
    env: &Env,
) -> (PayrollEscrowContractClient<'_>, Address, Map<Address, u32>) {
    let contract_id = env.register_contract(None, PayrollEscrowContract);
    let client = PayrollEscrowContractClient::new(env, &contract_id);

    let admin = Address::generate(env);
    let signer_1 = Address::generate(env);
    let signer_2 = Address::generate(env);

    let mut signers_weight_map: Map<Address, u32> = Map::new(env);
    signers_weight_map.set(signer_1, 1);
    signers_weight_map.set(signer_2, 1);

    client.initialize(&admin, &signers_weight_map, &2);
    (client, admin, signers_weight_map)
}

#[test]
fn test_contract_initialization_state() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, admin, signers_weight_map) = setup_initialized_contract(&env);

    // Admin and threshold persisted correctly.
    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_threshold(), 2);

    // Every signer entry written to instance storage.
    let stored = client.get_signers();
    for (addr, weight) in signers_weight_map.iter() {
        assert_eq!(stored.get(addr), Some(weight));
    }
}

#[test]
fn test_duplicate_initialization_is_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    // Separate contract registration so state is clean.
    let contract_id = env.register_contract(None, PayrollEscrowContract);
    let client = PayrollEscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let signer = Address::generate(&env);
    let mut map: Map<Address, u32> = Map::new(&env);
    map.set(signer, 1);

    client.initialize(&admin, &map, &1);

    let res = client.try_initialize(&admin, &map, &1);
    assert_eq!(
        res.unwrap_err().unwrap(),
        ContractError::AlreadyInitialized,
        "expected AlreadyInitialized on redundant initialization"
    );
}
