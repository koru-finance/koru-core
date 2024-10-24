use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DataKey {
    Admin,
    Token,
}