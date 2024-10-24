use soroban_sdk::{Address, BytesN, Env};

use super::types::storage::DataKey;

pub(crate) fn has_bridge_contract(env: &Env) -> bool {
    let key = DataKey::Bridge;
    env.storage().instance().has(&key)
}

pub(crate) fn set_bridge_contract(env: &Env, bridge_contract: Address) {
    let key = DataKey::Bridge;
    env.storage().instance().set(&key, &bridge_contract);
}

pub(crate) fn get_bridge_contract(env: &Env) -> Address {
    let key = DataKey::Bridge;
    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn has_token(env: &Env) -> bool {
    let key = DataKey::Token;
    env.storage().instance().has(&key)
}

pub(crate) fn set_token(env: &Env, token: Address) {
    let key = DataKey::Token;
    env.storage().instance().set(&key, &token);
}

pub(crate) fn get_token(env: &Env) -> Address {
    let key = DataKey::Token;
    env.storage().instance().get(&key).unwrap()
}



pub(crate) fn set_external_chain_token(env: &Env, token: BytesN<32>) {
    let key = DataKey::ExternalChainToken;
    env.storage().instance().set(&key, &token);
}

pub(crate) fn get_external_chain_token(env: &Env) -> BytesN<32> {
    let key = DataKey::ExternalChainToken;
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

pub(crate) fn set_deposit_address(env: &Env, address: Address) {
    let key = DataKey::DespositAddress;
    env.storage().instance().set(&key, &address);
}

pub(crate) fn get_deposit_address(env: &Env) -> Address {
    let key = DataKey::DespositAddress;
    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn set_protocol_address(env: &Env, address: Address) {
    let key = DataKey::ProtocolAddress;
    env.storage().instance().set(&key, &address);
}

pub(crate) fn get_protocol_address(env: &Env) -> Address {
    let key = DataKey::ProtocolAddress;
    env.storage().instance().get(&key).unwrap()
}
