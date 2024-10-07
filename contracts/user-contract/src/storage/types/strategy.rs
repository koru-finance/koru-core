use soroban_sdk::contracttype;

#[contracttype]
pub struct Strategy {
    pub id: u32,
    pub duration: u128,
    pub interest_rate: u128,
    pub pt: u128,
}
