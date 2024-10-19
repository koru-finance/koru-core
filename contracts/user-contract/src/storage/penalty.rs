use soroban_sdk::Env;

use super::types::storage::DataKey;

pub(crate) fn get_protocol_penalty(env: &Env) -> u128 {
    let key = DataKey::ProtocolPenalty;
    env.storage().instance().get(&key).unwrap_or(0)
}

pub(crate) fn set_protocol_penalty(env: &Env, penalty: u128) {
    let key = DataKey::ProtocolPenalty;
    env.storage().instance().set(&key, &penalty);
}

pub(crate) fn get_koru_penalty(env: &Env) -> u128 {
    let key = DataKey::KoruPenalty;
    env.storage().instance().get(&key).unwrap_or(0)
}

pub(crate) fn set_koru_penalty(env: &Env, penalty: u128) {
    let key = DataKey::KoruPenalty;
    env.storage().instance().set(&key, &penalty);
}