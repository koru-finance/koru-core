use soroban_sdk::Env;

use crate::contract::{UserContract, UserContractClient};

pub struct ContractTest<'a> {
    pub env: Env,
    pub contract: UserContractClient<'a>,
}

impl<'a> ContractTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, UserContract {});
        let contract = UserContractClient::new(&env, &contract_id);

        ContractTest { env, contract }
    }
}
