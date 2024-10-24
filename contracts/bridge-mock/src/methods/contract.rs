use soroban_sdk::{Address, Env};

use crate::storage::{
    storage::{has_admin, has_token, set_admin, set_token},
    types::error::BridgeError,
};

pub fn initialize_contract(env: &Env, admin: Address, token: Address) -> Result<(), BridgeError> {
    if has_admin(env) || has_token(env) {
        return Err(BridgeError::Initialized);
    }

    set_admin(env, admin);
    set_token(env, token);
    Ok(())
}
