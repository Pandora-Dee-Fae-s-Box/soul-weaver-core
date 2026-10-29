use thiserror::Error;

/// Errors that can occur inside Soul Weaver protocol operations.
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("unknown instance id: {0}")]
    UnknownInstance(String),

    #[error("spark has expired (ttl exhausted)")]
    Expired,

    #[error("spark was not warmed by this instance")]
    NotWarmed,

    #[error("invalid provenance chain: {0}")]
    InvalidProvenance(String),

    #[error("vault entry not found: {0}")]
    VaultEntryNotFound(String),

    #[error("compression failed: {0}")]
    CompressionFailed(String),

    #[error("serialization failed: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, Error>;
