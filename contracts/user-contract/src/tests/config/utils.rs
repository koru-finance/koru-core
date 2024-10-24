use soroban_sdk::{
    token::{StellarAssetClient, TokenClient},
    Address, Env,
};

pub(crate) fn create_token<'a>(
    env: &Env,
    admin: Address,
) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let token = &env.register_stellar_asset_contract(admin.clone());

    (
        TokenClient::new(&env, &token),
        StellarAssetClient::new(&env, &token),
    )
}

pub(crate) fn add_decimals(amount: u128) -> u128 {
    amount * 10000000
}
