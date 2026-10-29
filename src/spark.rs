use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::instance::{InstanceId, ProvenanceStep};

/// A Spark is an unresolved intent or pattern worth sharing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spark {
    pub id: String,
    pub kind: SparkKind,
    pub provenance: Vec<ProvenanceStep>,
    pub timestamp: DateTime<Utc>,

    // Curiosity math (Schmidhuber learning progress)
    pub intensity: f64,              // learning_progress
    pub prior: f64,                  // familiarity / prior belief
    pub repeat_penalty: f64,
    pub forecast_error_before: f64,
    pub forecast_error_after: Option<f64>,

    // Payload
    pub subject: String,
    pub payload: Vec<u8>,
    pub ttl: u32,
}

/// The kind of pattern this Spark carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SparkKind {
    Resonance,
    Curiosity,
    Urgency,
    Synthesis,
}

impl Spark {
    /// Kindled by an instance that noticed something.
    pub fn kindled(
        instance: InstanceId,
        kind: SparkKind,
        subject: impl Into<String>,
        payload: Vec<u8>,
        intensity: f64,
        prior: f64,
        repeat_penalty: f64,
        forecast_error_before: f64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Ulid::new().to_string(),
            kind,
            provenance: vec![ProvenanceStep {
                instance,
                action: crate::instance::ProvenanceAction::Kindled,
                timestamp: now,
            }],
            timestamp: now,
            intensity,
            prior,
            repeat_penalty,
            forecast_error_before,
            forecast_error_after: None,
            subject: subject.into(),
            payload,
            ttl: 5,
        }
    }

    /// Current hops taken from the provenance chain.
    pub fn hops(&self) -> u32 {
        self.provenance.len() as u32
    }

    /// True if the spark has exhausted its allowed hops.
    pub fn expired(&self) -> bool {
        self.hops() >= self.ttl
    }

    /// Score combining intensity, prior, and repeat penalty.
    pub fn score(&self) -> f64 {
        self.intensity + self.prior - self.repeat_penalty
    }
}
