use soroban_sdk::{Address, BytesN, Env, Symbol};

use crate::{
    shared::constants::BRIDGE_FUNCTION,
    storage::{
        storage::{get_bridge_contract, get_token},
        types::bridge_error::BridgeError,
    },
};

use super::build_args::build_args;

pub fn bridge(
    env: &Env,
    from: Address,
    receiver: BytesN<32>,
    amount: u128,
    bridge_fee: u128,
) -> Result<(), BridgeError> {
    let bridge_contract_address = get_bridge_contract(env);
    let func = Symbol::new(&env, BRIDGE_FUNCTION);

    let token = get_token(&env);

    let args = build_args(&env, from, receiver, amount, token, bridge_fee);

    let result =
        env.invoke_contract::<Result<(), BridgeError>>(&bridge_contract_address, &func, args);

    result
}
