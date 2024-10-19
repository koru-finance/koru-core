use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use crate::{
    methods::{admin, contract, public},
    storage::{
        strategy::get_strategy,
        types::{contract_errors::ContractError, strategy::Strategy, transaction::Transaction},
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
        deposit_address: Address,
        protocol_address: Address,
    ) -> Result<(), ContractError> {
        contract::initialize(
            &env,
            admin,
            token,
            bridge_contract,
            external_chain_token,
            deposit_address,
            protocol_address,
        )
    }

    pub fn deposit(env: Env, borrower: Address, collateral: u128, strategy_id: u32) -> Transaction {
        borrower.require_auth();

        public::deposit_and_bridge(&env, borrower.clone(), collateral, strategy_id)
    }

    pub fn start_withdraw(
        env: Env,
        address: Address,
        start_period: u64,
        end_period: u64,
    ) -> Result<(), ContractError> {
        public::start_withdraw(&env, address, start_period, end_period)
    }

    pub fn update_withdraw_status(
        env: Env,
        address: Address,
        start_period: u64,
        end_period: u64,
        can_withdraw: bool,
    ) {
        admin::update_withdraw_status(&env, address, start_period, end_period, can_withdraw);
    }

    pub fn withdraw(
        env: Env,
        address: Address,
        start_period: u64,
        end_period: u64,
    ) -> Result<(), ContractError> {
        public::withdraw(&env, address, start_period, end_period)
    }

    pub fn get_strategy(env: Env, id: u32) -> Strategy {
        get_strategy(&env, &id)
    }

    pub fn get_transaction(
        env: Env,
        address: Address,
        start_period: u64,
        end_period: u64,
    ) -> Transaction {
        contract::get_transaction(&env, address, start_period, end_period)
    }

    pub fn store_strategy(env: Env, id: u32, duration: u64, interest_rate: u128, pt: u128) {
        contract::save_strategy(&env, id, duration, interest_rate, pt);
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        contract::upgrade_bytecode(&env, new_wasm_hash)
    }

    pub fn set_investor_premium(env: &Env, premium: u128) {
        admin::set_investor_premium(&env, premium);
    }

    pub fn set_protocol_premium(env: &Env, premium: u128) {
        admin::set_protocol_premium(&env, premium);
    }

    pub fn set_protocol_penalty(env: &Env, penalty: u128) {
        admin::set_protocol_penalty(&env, penalty);
    }

    pub fn set_koru_penalty(env: &Env, penalty: u128) {
        admin::set_koru_penalty(&env, penalty);
    }
}
