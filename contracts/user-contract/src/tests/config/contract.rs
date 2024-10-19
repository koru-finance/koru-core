use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{StellarAssetClient, TokenClient},
    Address, BytesN, Env,
};

use crate::contract::{UserContract, UserContractClient};

use super::utils:: create_token;

pub struct ContractTest<'a> {
    pub env: Env,
    pub contract: UserContractClient<'a>,
    pub admin: Address,
    pub deposit_address: Address,
    pub protocol_address: Address,
    pub user_a: Address,
    pub bridge: Address,
    pub token_client: TokenClient<'a>,
    pub token_admin: StellarAssetClient<'a>,
}

impl<'a> ContractTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, UserContract {});
        let contract = UserContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let deposit_address = Address::generate(&env);
        let protocol_address = Address::generate(&env);
        let user_a = Address::generate(&env);
        let bridge = Address::generate(&env);

        let (token_client, token_admin) = create_token(&env, admin.clone());

        ContractTest {
            env,
            contract,
            admin,
            deposit_address,
            protocol_address,
            bridge,
            user_a,
            token_client,
            token_admin,
        }
    }

    pub fn initialize() -> Self {
        let config = ContractTest::setup();

        config.contract.initialize(
            &config.admin.clone(),
            &config.token_client.address.clone(),
            &config.bridge.clone(),
            &BytesN::from_array(&config.env, &[0; 32]),
            &config.deposit_address.clone(),
            &config.protocol_address.clone(),
        );

        config.contract.store_strategy(&1, &120, &12_000, &0);

        let strategy = config.contract.get_strategy(&1);

        assert_eq!(strategy.yield_rate, 12_000);

        ContractTest { ..config }
    }

    pub fn add_ledger_time(env: &Env, time: u64) {
        env.ledger().with_mut(|li| {
            li.timestamp += time;
        });
    }
}
