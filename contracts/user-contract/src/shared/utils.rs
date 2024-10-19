use soroban_sdk::{token::TokenClient, Address, BytesN, Env};

use crate::storage::storage;

pub fn str_to_bytesn32(env: &Env, input: &str) -> BytesN<32> {
    let mut bytes_array = [0u8; 32];
    let input_bytes = input.as_bytes();

    let len = input_bytes.len().min(32);
    bytes_array[..len].copy_from_slice(&input_bytes[..len]);

    BytesN::from_array(env, &bytes_array)
}

// Divide by 100_000 to get 3 decimal places of accuracy when calculating percentages
pub(crate) fn calculate_percentage(amount: u128, percentage: u128) -> u128 {
    (amount * percentage) / 100_000
}

pub fn transfer_token(env: &Env, from: &Address, to: &Address, amount: u128) {
    let token = storage::get_token(&env);

    let client = TokenClient::new(&env, &token);

    client.transfer(from, to, &(amount as i128));
}