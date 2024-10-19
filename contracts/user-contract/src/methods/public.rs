use soroban_sdk::{Address, BytesN, Env};

use crate::{
    events,
    shared::{
        constants::{DAYS_PER_YEAR, SECONDS_PER_DAY},
        utils,
    },
    storage::{
        penalty,
        premium::{get_investor_premium, get_protocol_premium},
        storage::{get_deposit_address, get_token},
        strategy::get_strategy,
        transaction,
        types::{contract_errors::ContractError, strategy::Strategy, transaction::Transaction},
    },
};

pub fn deposit_and_bridge(
    env: &Env,
    from: Address,
    collateral: u128,
    strategy_id: u32,
) -> Transaction {
    let strategy = get_strategy(env, &strategy_id);
    let token = get_token(env);

    let mut loan_to_collateral = strategy.yield_rate;

    let protocol_premium = get_protocol_premium(env);
    let investor_premium = get_investor_premium(env);

    if protocol_premium > 0 {
        loan_to_collateral += protocol_premium;
    }

    if investor_premium > 0 {
        loan_to_collateral += investor_premium;
    }

    let loaned_amount = utils::calculate_percentage(collateral, loan_to_collateral);

    utils::transfer_token(&env, &env.current_contract_address(), &from, loaned_amount);

    let deposit_address = get_deposit_address(&env);

    utils::transfer_token(env, &from, &deposit_address, collateral);
    

    let end_lock_up_period = strategy.duration * SECONDS_PER_DAY + env.ledger().timestamp();

    let transaction = Transaction {
        collateral,
        start_period: env.ledger().timestamp(),
        strategy: Strategy {
            id: strategy_id,
            duration: strategy.duration.clone(),
            yield_rate: strategy.yield_rate.clone(),
            pt: strategy.pt.clone(),
        },
        end_period: end_lock_up_period,
        loaned_amount: loaned_amount.clone(),
        from: from.clone(),
        token: token.clone(),
        to: BytesN::from_array(&env, &[0; 32]),
        investor_premium,
        protocol_premium,
        total_apy: loan_to_collateral.clone(),
        can_withdraw: false,
        is_withdrawn: false,
    };

    transaction::set(&env, &transaction);

    transaction
}

pub(crate) fn start_withdraw(
    env: &Env,
    address: Address,
    start_period: u64,
    end_period: u64,
) -> Result<(), ContractError> {
    if !transaction::has(&env, address.clone(), start_period, end_period) {
        return Err(ContractError::InvalidTransaction);
    }

    address.require_auth();

    events::contract::emit_withdraw(env, address, start_period, end_period);

    Ok(())
}

pub(crate) fn withdraw(
    env: &Env,
    address: Address,
    start_period: u64,
    end_period: u64,
) -> Result<(), ContractError> {
    if !transaction::has(&env, address.clone(), start_period, end_period) {
        return Err(ContractError::InvalidTransaction);
    }

    address.require_auth();

    let mut transaction = transaction::get(&env, address, start_period, end_period);

    if !transaction.can_withdraw {
        return Err(ContractError::Unauthorized);
    }

    if transaction.end_period > env.ledger().timestamp() {
        let protocol_penalty_percentage = penalty::get_protocol_penalty(&env);
        let koru_penalty_percentage = penalty::get_koru_penalty(&env);

        let protocol_penalty =
            utils::calculate_percentage(transaction.loaned_amount, protocol_penalty_percentage);
        let koru_penalty =
            utils::calculate_percentage(transaction.loaned_amount, koru_penalty_percentage);

        let remaining_lockup_days =
            (transaction.end_period - env.ledger().timestamp()) / SECONDS_PER_DAY;

        let investors_yield_due = ((((transaction.loaned_amount * remaining_lockup_days as u128)
            / DAYS_PER_YEAR as u128) * transaction.total_apy) / 100_000) / 10_000;

        let mut loan_penalty =
            utils::calculate_percentage(transaction.loaned_amount, investors_yield_due);

        loan_penalty += protocol_penalty;
        loan_penalty += koru_penalty;
        
        utils::transfer_token(
            &env,
            &env.current_contract_address(),
            &transaction.from,
            transaction.collateral - loan_penalty,
        );
    } else {
        utils::transfer_token(
            &env,
            &env.current_contract_address(),
            &transaction.from,
            transaction.collateral,
        );
    }

    transaction.is_withdrawn = true;
    transaction.can_withdraw = false;
    transaction::set(&env, &transaction);

    Ok(())
}
