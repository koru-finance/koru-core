use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, U256};

use crate::methods::bridge::swap_and_bridge as bridge_founds;
use crate::methods::contract::initialize_contract;
use crate::storage::types::error::BridgeError;

#[contract]
pub struct BridgeMock {}

#[contractimpl]
impl BridgeMock {
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), BridgeError> {
        initialize_contract(&env, admin, token)
    }

    pub fn swap_and_bridge(
        env: Env,
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
        sender.require_auth();

        bridge_founds(
            &env,
            sender,
            token,
            amount,
            recipient,
            destination_chain_id,
            receive_token,
            nonce,
            gas_amount,
            fee_token_amount,
        )
    }
}
