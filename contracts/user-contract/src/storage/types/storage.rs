use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Bridge,
    DespositAddress,
    ExternalChainToken,
    KoruPenalty,
    InvestorPremium,
    Premium(Address),
    ProtocolAddress,
    ProtocolPremium,
    ProtocolPenalty,
    Strategy(u32),
    Token,
    Transaction((Address, u64, u64)),
    User(Address),
}