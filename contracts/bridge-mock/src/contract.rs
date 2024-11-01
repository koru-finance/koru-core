use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, U256};

use crate::methods::bridge;
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

        bridge::swap_and_bridge(
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

    pub fn receive_tokens(
        env: Env,
        sender: Address,
        amount: u128,
        recipient: Address,
        source_chain_id: u32,
        receive_token: BytesN<32>,
        nonce: U256,
        receive_amount_min: u128,
        extra_gas: Option<u128>,
    ) -> Result<(), BridgeError> {
        sender.require_auth();

        bridge::receive_tokens(
            &env,
            sender,
            amount,
            recipient,
            source_chain_id,
            receive_token,
            nonce,
            receive_amount_min,
            extra_gas
        )
    }
}
