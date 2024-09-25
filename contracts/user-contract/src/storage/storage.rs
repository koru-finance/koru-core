use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub(crate) fn has_bridge_contract(env: &Env) -> bool {
    let key = DataKey::Bridge;
    env.storage().instance().has(&key)
}

pub(crate) fn set_bridge_contract(env: &Env) {
    let key = DataKey::Bridge;
    env.storage().instance().set(&key, &true);
}

pub(crate) fn get_bridge_contract(env: &Env) -> Address {
    let key = DataKey::Bridge;
    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn has_token(env: &Env) -> bool {
    let key = DataKey::Token;
    env.storage().instance().has(&key)
}

pub(crate) fn set_token(env: &Env) {
    let key = DataKey::Token;
    env.storage().instance().set(&key, &true);
}

pub(crate) fn get_token(env: &Env) -> Address {
    let key = DataKey::Token;
    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;
    env.storage().instance().has(&key)
}

pub(crate) fn set_admin(env: &Env, admin: Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, &admin);
}

pub(crate) fn get_admin(env: &Env) -> Address {
    let key = DataKey::Admin;
    env.storage().instance().get(&key).unwrap()
}
