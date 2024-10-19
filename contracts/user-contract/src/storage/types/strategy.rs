use soroban_sdk::contracttype;

#[contracttype]
pub struct Strategy {
    pub id: u32,
    pub duration: u64,
    pub yield_rate: u128,
    pub pt: u128,
}
