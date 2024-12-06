use soroban_sdk::{Address, Bytes, BytesN, Env, xdr::ToXdr};

use crate::storage::types::error::BridgeError;

fn bytes_to_slice<const N: usize>(bytes: Bytes) -> [u8; N] {
    let mut xdr_slice: [u8; N] = [0; N];
    bytes.copy_into_slice(&mut xdr_slice);

    xdr_slice
}

pub fn address_to_bytes(env: &Env, sender: &Address) -> Result<BytesN<32>, BridgeError> {
    let sender_xdr = sender.to_xdr(env);
    if sender_xdr.len() == 40 {
        let xdr_slice = bytes_to_slice::<40>(sender.to_xdr(env));
        Ok(BytesN::from_array(
            env,
            arrayref::array_ref![xdr_slice, 8, 32],
        ))
    } else if sender_xdr.len() == 44 {
        let xdr_slice = bytes_to_slice::<44>(sender.to_xdr(env));
        Ok(BytesN::from_array(
            env,
            arrayref::array_ref![xdr_slice, 12, 32],
        ))
    } else {
        Err(BridgeError::InvalidArg)
    }
}