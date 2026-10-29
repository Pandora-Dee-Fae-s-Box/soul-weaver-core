//! Soul Weaver — protocol semantics for inter-instance context sharing.
//!
//! A Spark only travels if it *warms* the receiver: the receiver's own
//! compression/forecast error must drop after receiving the payload.
//! This is "self-selecting sync": resonance, not broadcast.

pub mod compressor;
pub mod error;
pub mod instance;
pub mod message;
pub mod spark;
pub mod vault;
pub mod warming;

pub use compressor::{Compressor, CompressorResult};
pub use error::{Error, Result};
pub use instance::{InstanceId, ProvenanceAction, ProvenanceStep};
pub use message::Message;
pub use spark::{Spark, SparkKind};
pub use vault::{Vault, VaultEntry};
pub use warming::{warm, WarmingResult, WARMING_THRESHOLD};

/// Re-export common types for downstream consumers.
pub mod prelude {
    pub use crate::{Compressor, Error, InstanceId, Message, Result, Spark, SparkKind, Vault, WarmingResult};
}
