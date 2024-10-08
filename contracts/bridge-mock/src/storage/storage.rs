use soroban_sdk::{Address, Env};

use super::types::data_key::DataKey;

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: Address) {
    env.storage().instance().set(&DataKey::Admin, &admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn has_token(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Token)
}

pub fn set_token(env: &Env, token: Address) {
    env.storage().instance().set(&DataKey::Token, &token);
}

pub fn get_token(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Token).unwrap()
}