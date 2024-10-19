use soroban_sdk::{Address, BytesN, Env};

use crate::storage::{
    storage::{
        get_admin, has_admin, set_admin, set_bridge_contract, set_deposit_address,
        set_external_chain_token, set_protocol_address, set_token,
    },
    strategy::{get_strategy, has_strategy, set_strategy},
    transaction,
    types::{contract_errors::ContractError, strategy::Strategy, transaction::Transaction},
};

pub fn save_strategy(env: &Env, id: u32, duration: u64, yield_rate: u128, pt: u128) {
    let admin = get_admin(&env);
    admin.require_auth();

    if !has_strategy(&env, &id) {
        let strategy = Strategy {
            id,
            duration,
            yield_rate,
            pt,
        };

        set_strategy(env, &id, strategy);
    } else {
        let mut stored_strategy = get_strategy(&env, &id);
        stored_strategy.duration = duration;
        stored_strategy.yield_rate = yield_rate;
        stored_strategy.pt = pt;

        set_strategy(env, &id, stored_strategy);
    }
}

pub fn initialize(
    env: &Env,
    admin: Address,
    token: Address,
    bridge_contract: Address,
    external_chain_token: BytesN<32>,
    deposit_address: Address,
    protocol_address: Address,
) -> Result<(), ContractError> {
    if has_admin(&env) {
        return Err(ContractError::AlreadyInitialized);
    }

    set_admin(env, admin);
    set_token(env, token);
    set_bridge_contract(env, bridge_contract);
    set_external_chain_token(env, external_chain_token);
    set_deposit_address(env, deposit_address);
    set_protocol_address(env, protocol_address);

    Ok(())
}

pub fn upgrade_bytecode(env: &Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
    if !has_admin(&env) {
        return Err(ContractError::NotInitialized);
    }

    let admin: Address = get_admin(&env);
    admin.require_auth();

    env.deployer().update_current_contract_wasm(new_wasm_hash);

    Ok(())
}

pub(crate) fn get_transaction(
    env: &Env,
    address: Address,
    start_period: u64,
    end_period: u64,
) -> Transaction {
    transaction::get(&env, address, start_period, end_period)
}
