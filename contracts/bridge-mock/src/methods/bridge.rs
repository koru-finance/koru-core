use soroban_sdk::{token as TokenClient, Address, BytesN, Env, U256};

use crate::{
    events::bridge::emitt_swap_and_bridge,
    storage::{
        storage::{get_token, has_admin},
        types::error::BridgeError,
    },
};

pub fn swap_and_bridge(
    env: &Env,
    sender: Address,
    token: Address,
    amount: u128,
    recipient: BytesN<32>,
    destination_chain_id: u32,
    receive_token: BytesN<32>,
    nonce: U256,
    gas_amount: u128,
    fee_token_amount: u128,
) -> Result<(), BridgeError> {
    if !has_admin(&env) {
        return Err(BridgeError::Uninitialized);
    }

    let stable_token = get_token(&env);

    if token != stable_token {
        return Err(BridgeError::UnknownToken);
    }

    let client = TokenClient::Client::new(&env, &token);
    let mut total_transfer_amount = amount as i128 + fee_token_amount as i128;

    if total_transfer_amount < 0 {
        total_transfer_amount = total_transfer_amount * -1;
    }

    client.transfer(
        &sender,
        &env.current_contract_address(),
        &total_transfer_amount.abs(),
    );

    emitt_swap_and_bridge(
        env,
        sender,
        token,
        amount,
        recipient,
        destination_chain_id,
        receive_token,
        nonce,
        gas_amount,
        fee_token_amount,
    );

    Ok(())
}
