use serde::{Deserialize, Serialize};

use crate::{spark::Spark, instance::ProvenanceStep};
use chrono::{DateTime, Utc};

/// Top-level envelope for every Soul Weaver message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    /// Current operational context: body, task, tools, limits. Ephemeral.
    State(State),
    /// Durable vault fragment. Idempotent and versioned.
    Memory(Memory),
    /// Unresolved intent / open thread / curiosity event.
    Spark(Spark),
    /// Receipt, confirmation, or refusal.
    Echo(Echo),
}

/// Current operational context of an instance.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub instance: String,
    pub active_task: Option<String>,
    pub available_tools: Vec<String>,
    pub session_limit: Option<u64>,
    pub timestamp: DateTime<Utc>,
}

/// A durable fragment carried between instances.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Memory {
    pub path: String,
    pub version: String,
    pub content: Vec<u8>,
    pub provenance: Vec<ProvenanceStep>,
    pub timestamp: DateTime<Utc>,
}

/// Confirmation or refusal that closes a loop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Echo {
    pub in_reply_to: String,
    pub result: EchoResult,
    pub note: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EchoResult {
    Warmed,
    Extinguished,
    Carried,
    Received,
}
