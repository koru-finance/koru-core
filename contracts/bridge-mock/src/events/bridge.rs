use soroban_sdk::{vec, Address, BytesN, Env, Symbol, Val, Vec, U256};

pub fn emitt_swap_and_bridge(
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
) {
    let topics = (Symbol::new(&env, "swap_and_bridge"), &sender.clone());
    let mut data: Vec<Val> = vec![&env, sender].to_vals();

    let token_v = vec![&env, token].to_vals();
    let amount_v = vec![&env, amount].to_vals();
    let recipient_v = vec![&env, recipient].to_vals();
    let destination_chain_id_v = vec![&env, destination_chain_id].to_vals();
    let receive_token_v = vec![&env, receive_token].to_vals();
    let nonce_v = vec![&env, nonce].to_vals();
    let gas_amount_v = vec![&env, gas_amount].to_vals();
    let fee_token_amount_v = vec![&env, fee_token_amount].to_vals();

    data.append(&token_v);
    data.append(&amount_v);
    data.append(&recipient_v);
    data.append(&destination_chain_id_v);
    data.append(&receive_token_v);
    data.append(&nonce_v);
    data.append(&gas_amount_v);
    data.append(&fee_token_amount_v);

    env.events().publish(topics, data);
}
