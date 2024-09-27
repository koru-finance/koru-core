use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    User(Address),
    Bridge,
    ExternalChainToken,
    Token,
    Strategy(u32),
}