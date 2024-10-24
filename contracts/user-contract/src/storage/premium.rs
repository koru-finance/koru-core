use soroban_sdk::Env;

use super::types::storage::DataKey;

pub(crate) fn get_protocol_premium(env: &Env) -> u128 {
    let key = DataKey::ProtocolPremium;
    env.storage().instance().get(&key).unwrap_or(0)
}

pub(crate) fn set_protocol_premium(env: &Env, premium: u128) {
    let key = DataKey::ProtocolPremium;
    env.storage().instance().set(&key, &premium);
}

pub(crate) fn get_investor_premium(env: &Env) -> u128 {
    let key = DataKey::InvestorPremium;
    env.storage().instance().get(&key).unwrap_or(0)
}

pub(crate) fn set_investor_premium(env: &Env, premium: u128) {
    let key = DataKey::InvestorPremium;
    env.storage().instance().set(&key, &premium);
}