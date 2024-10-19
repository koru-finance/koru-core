use soroban_sdk::{Address, Env};

use crate::storage::{penalty, premium, storage::{get_admin as get_contract_admin, set_admin as set_contract_admin}, transaction};


pub(crate) fn set_investor_premium(env: &Env, premium: u128) {
    let admin = get_admin(&env);
    admin.require_auth();

    premium::set_investor_premium(&env, premium);
}

pub(crate) fn set_protocol_premium(env: &Env, premium: u128) {
    let admin = get_admin(&env);
    admin.require_auth();

    premium::set_protocol_premium(&env, premium);
}

pub(crate) fn set_koru_penalty(env: &Env, penalty: u128) {
    let admin = get_admin(&env);
    admin.require_auth();

    penalty::set_koru_penalty(&env, penalty);
}

pub(crate) fn set_protocol_penalty(env: &Env, penalty: u128) {
    let admin = get_admin(&env);
    admin.require_auth();

    penalty::set_protocol_penalty(&env, penalty);
}

pub(crate) fn update_withdraw_status(
    env: &Env,
    address: Address,
    start_period: u64,
    end_period: u64,
    can_withdraw: bool,
) {
    let admin = get_admin(&env);
    admin.require_auth();

    let mut transaction = transaction::get(&env, address, start_period, end_period);

    transaction.can_withdraw = can_withdraw;

    transaction::set(&env, &transaction);
}


pub fn set_admin(env: &Env, new_admin: Address) {
    let admin = get_contract_admin(env);

    admin.require_auth();

    set_contract_admin(env, new_admin);
}

pub fn get_admin(env: &Env) -> Address {
    get_contract_admin(env)
}
