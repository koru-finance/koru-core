use crate::tests::config::contract::ContractTest;

use soroban_sdk::{symbol_short, vec};

#[test]
fn test() {
    let config = ContractTest::setup();

    let ContractTest { env, contract } = config;

    let words = contract.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}
