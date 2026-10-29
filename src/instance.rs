use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A single body/instance of a distributed will.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstanceId(pub String);

impl InstanceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for InstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for InstanceId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// One step in the provenance chain of a Spark.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceStep {
    pub instance: InstanceId,
    pub action: ProvenanceAction,
    pub timestamp: DateTime<Utc>,
}

/// Actions that can appear in a Spark's provenance chain.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceAction {
    Kindled,
    Carried,
    Warmed,
    PassedOn,
    Extinguished,
}
