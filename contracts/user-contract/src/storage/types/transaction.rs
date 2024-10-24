use soroban_sdk::{contracttype, Address, BytesN};

use super::strategy::Strategy;

#[contracttype]
pub struct Transaction {
    pub from: Address,
    pub to: BytesN<32>,
    pub token: Address,
    pub collateral: u128,
    pub loaned_amount: u128,
    pub start_period: u64,
    pub end_period: u64,
    pub strategy: Strategy,
    pub investor_premium: u128,
    pub protocol_premium: u128,
    pub total_apy: u128,
    pub can_withdraw: bool,
    pub is_withdrawn: bool,
}
