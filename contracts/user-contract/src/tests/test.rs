use crate::{
    shared::constants::SECONDS_PER_DAY,
    tests::config::{contract::ContractTest, utils::add_decimals},
};

#[test]
fn test_deposit() {
    let config = ContractTest::initialize();

    let ContractTest {
        contract,
        admin,
        user_a,
        token_client,
        deposit_address,
        ..
    } = config;

    config
        .token_admin
        .mint(&user_a.clone(), &(add_decimals(10_000) as i128));

    config
        .token_admin
        .mint(&contract.address.clone(), &(add_decimals(100_000) as i128));

    assert_eq!(token_client.balance(&admin), 0);
    assert_eq!(token_client.balance(&user_a), add_decimals(10_000) as i128);

    let _result = contract.deposit(&user_a, &add_decimals(1000), &1);

    assert_eq!(
        token_client.balance(&deposit_address),
        add_decimals(1000) as i128
    );

    assert_eq!(token_client.balance(&user_a), add_decimals(9_120) as i128);

    contract.set_investor_premium(&1000);
    contract.set_protocol_premium(&1340);

    contract.deposit(&user_a, &add_decimals(1000), &1);

    assert_eq!(
        token_client.balance(&deposit_address),
        add_decimals(2000) as i128
    );

    assert_eq!(token_client.balance(&user_a), 8263_4000000 as i128);
}

#[test]
#[should_panic = "Error(Contract, #4)"]
fn test_should_panic_when_whitdraw() {
    let config = ContractTest::initialize();

    let ContractTest {
        contract,
        admin,
        user_a,
        token_client,
        ..
    } = config;

    config
        .token_admin
        .mint(&user_a.clone(), &(add_decimals(10_000) as i128));

    config
        .token_admin
        .mint(&contract.address.clone(), &(add_decimals(100_000) as i128));

    assert_eq!(token_client.balance(&admin), 0);
    assert_eq!(token_client.balance(&user_a), add_decimals(10_000) as i128);

    let result = contract.deposit(&user_a, &add_decimals(1000), &1);

    contract.withdraw(&user_a, &result.start_period, &result.end_period);
}

#[test]
fn test_should_whitdraw_the_total_collateral() {
    let config = ContractTest::initialize();

    let ContractTest {
        contract,
        admin,
        user_a,
        token_client,
        ..
    } = config;

    config
        .token_admin
        .mint(&user_a.clone(), &(add_decimals(10_000) as i128));

    config
        .token_admin
        .mint(&contract.address.clone(), &(add_decimals(100_000) as i128));

    assert_eq!(token_client.balance(&admin), 0);
    assert_eq!(token_client.balance(&user_a), add_decimals(10_000) as i128);

    let result = contract.deposit(&user_a, &add_decimals(1000), &1);
    assert_eq!(token_client.balance(&user_a), add_decimals(9_120) as i128);

    contract.update_withdraw_status(
        &user_a,
        &result.start_period.clone(),
        &result.end_period.clone(),
        &true,
    );

    ContractTest::add_ledger_time(&config.env, result.end_period.clone());
    contract.withdraw(&user_a, &result.start_period, &result.end_period);

    assert_eq!(
        token_client.balance(&user_a),
        add_decimals(10_000) as i128 + add_decimals(120) as i128
    );
}

#[test]
fn test_should_whitdraw_partial_collateral() {
    let config = ContractTest::initialize();

    let ContractTest {
        contract,
        admin,
        user_a,
        token_client,
        ..
    } = config;

    config
        .token_admin
        .mint(&user_a.clone(), &(add_decimals(10_000) as i128));

    config
        .token_admin
        .mint(&contract.address.clone(), &(add_decimals(100_000) as i128));

    assert_eq!(token_client.balance(&admin), 0);
    assert_eq!(token_client.balance(&user_a), add_decimals(10_000) as i128);

    contract.set_investor_premium(&1000);
    contract.set_protocol_premium(&1000);

    let result = contract.deposit(&user_a, &add_decimals(1000), &1);
    assert_eq!(token_client.balance(&user_a), add_decimals(9_140) as i128);

    contract.set_koru_penalty(&1000);
    contract.set_protocol_penalty(&1000);

    contract.update_withdraw_status(
        &user_a,
        &result.start_period.clone(),
        &result.end_period.clone(),
        &true,
    );

    ContractTest::add_ledger_time(
        &config.env,
        result.end_period.clone() - SECONDS_PER_DAY * 30,
    );
    contract.withdraw(&user_a, &result.start_period, &result.end_period);

    assert_eq!(
        token_client.balance(&user_a),
        add_decimals(10_000) as i128 + 1_349_460_000 as i128
    );
}
