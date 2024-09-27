use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use crate::{
    methods::{contract::save_strategy, public::deposit_and_bridge},
    storage::{
        strategy::get_strategy,
        types::strategy::Strategy,
    },
};

#[contract]
pub struct UserContract;

#[contractimpl]
impl UserContract {
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
}
