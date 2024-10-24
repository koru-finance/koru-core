use soroban_sdk::{Address, BytesN, Env};

use crate::storage::{
    storage::{has_admin, set_admin, set_bridge_contract, set_external_chain_token, set_token},
    strategy::{get_strategy, has_strategy, set_strategy},
    types::{contract_errors::ContractError, strategy::Strategy},
};

pub fn save_strategy(env: &Env, id: u32, duration: u128, interest_rate: u128, pt: u128) {
    if !has_strategy(&env, &id) {
        let strategy = Strategy {
            id,
            duration,
            interest_rate,
            pt,
        };

        set_strategy(env, &id, strategy);
    } else {
        let mut stored_strategy = get_strategy(&env, &id);
        stored_strategy.duration = duration;
        stored_strategy.interest_rate = interest_rate;
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
) -> Result<(), ContractError> {
    if !has_admin(&env) {
        return Err(ContractError::AlreadyInitialized);
    }

    set_admin(env, admin);
    set_token(env, token);
    set_bridge_contract(env, bridge_contract);
    set_external_chain_token(env, external_chain_token);

    Ok(())
}
