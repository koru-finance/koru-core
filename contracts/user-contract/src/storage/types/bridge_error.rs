use soroban_sdk::contracterror;


#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BridgeError {
    UnauthorizedStopAuthority = 203,
    SwapProhibited = 204,
    AmountTooLowForFee = 205,
    BridgeToTheZeroAddress = 206,
    EmptyRecipient = 207,
    SourceNotRegistered = 208,
    WrongDestinationChain = 209,
    UnknownAnotherChain = 210,
    TokensAlreadySent = 211,
    MessageProcessed = 212,
    NotEnoughFee = 214,
    NoMessage = 215,
    NoReceivePool = 216,
    NoPool = 217,
    UnknownAnotherToken = 218,
}