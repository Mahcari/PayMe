#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Map};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Admin,
    Signers,
    Threshold,
    Initialized,
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    EmptySigners = 2,
    InvalidThreshold = 3,
}

#[contract]
pub struct PayrollEscrowContract;

#[contractimpl]
impl PayrollEscrowContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        signers_weight_map: Map<Address, u32>,
        required_threshold: u32,
    ) -> Result<(), ContractError> {
        if env
            .storage()
            .instance()
            .get::<_, bool>(&DataKey::Initialized)
            .unwrap_or(false)
        {
            return Err(ContractError::AlreadyInitialized);
        }

        if signers_weight_map.is_empty() {
            return Err(ContractError::EmptySigners);
        }

        if required_threshold == 0 {
            return Err(ContractError::InvalidThreshold);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::Signers, &signers_weight_map);
        env.storage()
            .instance()
            .set(&DataKey::Threshold, &required_threshold);
        env.storage().instance().set(&DataKey::Initialized, &true);

        Ok(())
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("admin is not initialized")
    }

    pub fn get_threshold(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Threshold)
            .expect("threshold is not initialized")
    }

    pub fn get_signers(env: Env) -> Map<Address, u32> {
        env.storage()
            .instance()
            .get(&DataKey::Signers)
            .expect("signers are not initialized")
    }
}

mod test;
