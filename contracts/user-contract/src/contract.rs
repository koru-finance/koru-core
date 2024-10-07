use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use crate::{
    methods::{
        contract::{initialize, save_strategy, upgrade_bytecode},
        public::deposit_and_bridge,
    },
    storage::{
        strategy::get_strategy,
        types::{contract_errors::ContractError, strategy::Strategy},
    },
};

#[contract]
pub struct UserContract;

#[contractimpl]
impl UserContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
        bridge_contract: Address,
        external_chain_token: BytesN<32>,
    ) -> Result<(), ContractError> {
        initialize(&env, admin, token, bridge_contract, external_chain_token)
    }

    pub fn deposit(env: Env, from: Address, receiver: BytesN<32>, amount: u128, extra_fee: u128) {
        from.require_auth();

        deposit_and_bridge(&env, from.clone(), receiver.clone(), amount, extra_fee);
    }

    pub fn store_strategy(env: Env, id: u32, duration: u128, interest_rate: u128, pt: u128) {
        save_strategy(&env, id, duration, interest_rate, pt);
    }

    pub fn get_strategy(env: Env, id: u32) -> Strategy {
        get_strategy(&env, &id)
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        upgrade_bytecode(&env, new_wasm_hash)
    }
}
