use soroban_sdk::{Address, Env};

use super::types::{storage::DataKey, transaction::Transaction};

pub(crate) fn has(
    env: &Env,
    address: Address,
    start_period: u64,
    end_period: u64,
) -> bool {
    let key = DataKey::Transaction((address, start_period, end_period));

    env.storage().instance().has(&key)
}

pub(crate) fn set(env: &Env, transaction: &Transaction) {
    let key = DataKey::Transaction((
        transaction.from.clone(),
        transaction.start_period.clone(),
        transaction.end_period.clone(),
    ));

    env.storage().instance().set(&key, transaction);
}

pub(crate) fn get(
    env: &Env,
    address: Address,
    start_period: u64,
    end_period: u64,
) -> Transaction {
    let key = DataKey::Transaction((address, start_period, end_period));

    env.storage().instance().get(&key).unwrap()
}
