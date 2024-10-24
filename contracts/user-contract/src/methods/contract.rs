use soroban_sdk::Env;

use crate::storage::{
    strategy::{get_strategy, has_strategy, set_strategy},
    types::strategy::Strategy,
};

pub fn save_strategy(env: &Env, id: u32, duration: u128, interest_rate: u128, pt: u128) {
    if !has_strategy(&env, &id) {
        let strategy = Strategy {
            id,
            duration,
            interest_rate,
            pt,
        };

        set_strategy(env, &id, strategy);
    } else {
        let mut stored_strategy = get_strategy(&env, &id);
        stored_strategy.duration = duration;
        stored_strategy.interest_rate = interest_rate;
        stored_strategy.pt = pt;

        set_strategy(env, &id, stored_strategy);
    }
}
