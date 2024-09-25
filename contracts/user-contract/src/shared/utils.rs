use soroban_sdk::{BytesN, Env};

pub fn str_to_bytesn32(env: &Env, input: &str) -> BytesN<32> {
    let mut bytes_array = [0u8; 32];
    let input_bytes = input.as_bytes();

    let len = input_bytes.len().min(32);
    bytes_array[..len].copy_from_slice(&input_bytes[..len]);

    BytesN::from_array(env, &bytes_array)
}