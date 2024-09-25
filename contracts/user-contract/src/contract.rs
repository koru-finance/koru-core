use soroban_sdk::{contract, contractimpl,  Address, BytesN, Env,};

use crate::methods::public::deposit_and_bridge;

#[contract]
pub struct UserContract;

#[contractimpl]
impl UserContract {
    pub fn deposit(env: Env, from: Address, receiver: BytesN<32>, amount: u128, extra_fee: u128) {
        from.require_auth();

        deposit_and_bridge(&env, from, receiver, amount, extra_fee);
    }
}
