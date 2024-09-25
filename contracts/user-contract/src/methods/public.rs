use soroban_sdk::{Address, BytesN, Env};

use crate::bridge::gateway::bridge;

pub fn deposit_and_bridge(
    env: &Env,
    from: Address,
    receiver: BytesN<32>,
    amount: u128,
    bridge_fee: u128,
) {
    let _ = bridge(env, from, receiver, amount, bridge_fee);
}
