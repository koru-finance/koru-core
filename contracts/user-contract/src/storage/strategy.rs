use soroban_sdk::Env;

use super::types::{storage::DataKey, strategy::Strategy};

pub(crate) fn has_strategy(env: &Env, id: &u32) -> bool {
    let key = DataKey::Strategy(id.clone());

    env.storage().instance().has(&key)
}

pub(crate) fn set_strategy(env: &Env, id: &u32, strategy: Strategy) {
    let key = DataKey::Strategy(id.clone());
    env.storage().instance().set(&key, &strategy);
}

pub(crate) fn get_strategy(env: &Env, id: &u32) -> Strategy {
    let key = DataKey::Strategy(id.clone());
    env.storage().instance().get(&key).unwrap()
}
