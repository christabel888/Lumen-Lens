use soroban_sdk::{contracterror, log, Env};

/// Contract-specific errors for Stellar Insights Analytics Contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Contract has already been initialized
    AlreadyInitialized = 1,
    /// Contract has not been initialized
    NotInitialized = 2,
    /// Caller is not authorized to perform this action
    Unauthorized = 3,
    /// Generic invalid epoch
    InvalidEpoch = 4,
    /// Epoch must be greater than 0
    InvalidEpochZero = 5,
    /// Epoch exceeds maximum allowed value
    InvalidEpochTooLarge = 6,
    /// Snapshot for this epoch already exists
    DuplicateEpoch = 7,
    /// Epoch must be strictly greater than the latest recorded epoch
    EpochMonotonicityViolated = 8,
    /// Contract is paused
    ContractPaused = 9,
    /// Contract is not paused
    ContractNotPaused = 10,
    /// Generic invalid hash
    InvalidHash = 11,
    /// Hash must not be all zeros
    InvalidHashZero = 12,
    /// No snapshot found for the requested epoch
    SnapshotNotFound = 6,
    /// Contract is paused for emergency maintenance
    ContractPaused = 7,
    /// Epoch must be strictly greater than latest (monotonicity violated)
    EpochMonotonicityViolated = 8,
}
