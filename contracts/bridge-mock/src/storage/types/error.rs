use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BridgeError {
    // Common
    Unimplemented = 0,
    Initialized = 1,
    Uninitialized = 2,
    Unauthorized = 3,
    InvalidArg = 4,
    InvalidChainId = 5,
    InvalidOtherChainId = 6,
    GasUsageNotSet = 7,
    BrokenAddress = 8,
    NotFound = 9,
    TokenInsufficientBalance = 10,
    CastFailed = 11,
    U256Overflow = 12,

    // Pool
    ZeroAmount = 103,
    PoolOverflow = 104,
    ZeroChanges = 105,
    ReservesExhausted = 106,
    InsufficientReceivedAmount = 107,
    BalanceRatioExceeded = 108,
    Forbidden = 109,

    // Bridge
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

    // Messenger
    WrongByteLength = 300,
    HasMessage = 301,
    InvalidPrimarySignature = 302,
    InvalidSecondarySignature = 303,

    // Gas Oracle
    NoGasDataForChain = 400,

    // Contract
    UnknownToken = 500,
}
