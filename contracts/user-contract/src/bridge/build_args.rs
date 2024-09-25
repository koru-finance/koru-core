use soroban_sdk::{vec, Address, BytesN, Env, Val, Vec, U256};

use crate::shared::{
    constants::{BRIDGE_CHAIN_ID, EXT_RECEIVER_TOKEN_ADDRESS},
    utils::str_to_bytesn32,
};

pub fn build_args(
    env: &Env,
    from: Address,
    recipient: BytesN<32>,
    amount: u128,
    token: Address,
    bridge_fee: u128,
) -> Vec<Val> {
    let mut fn_args = vec![&env, from].to_vals();
    let source_token_vals = vec![&env, token].to_vals();
    let recipient_vals = vec![&env, recipient].to_vals();
    let chain_id_vals = vec![&env, BRIDGE_CHAIN_ID].to_vals();
    let amount_vals = vec![&env, amount].to_vals();
    let receiver_token_bytes = str_to_bytesn32(&env, EXT_RECEIVER_TOKEN_ADDRESS);
    let ext_receiver_token_vals = vec![&env, receiver_token_bytes].to_vals();

    let nonce_vals = vec![
        &env,
        U256::from_u128(&env, env.ledger().timestamp() as u128),
    ]
    .to_vals();
    let gas_amount_vals = vec![&env, 0].to_vals();

    let bridge_fee_vals = vec![&env, bridge_fee].to_vals();

    fn_args.append(&source_token_vals);
    fn_args.append(&amount_vals);
    fn_args.append(&recipient_vals);
    fn_args.append(&chain_id_vals);
    fn_args.append(&ext_receiver_token_vals);
    fn_args.append(&nonce_vals);
    fn_args.append(&gas_amount_vals);
    fn_args.append(&bridge_fee_vals);

    fn_args
}
