use soroban_sdk::{vec, Address, Env, Symbol, Val, Vec};

pub(crate) fn emit_withdraw(env: &Env, address: Address, start_period: u64, end_period: u64) {

    let topics = (Symbol::new(&env, "initialize_withdraw"), &address.clone());

    let mut data: Vec<Val> = vec![&env, address].to_vals();
    let start_period_v = vec![&env, start_period].to_vals();
    let end_period_v = vec![&env, end_period].to_vals();

    data.append(&start_period_v);
    data.append(&end_period_v);


    env.events().publish(topics, data);
}