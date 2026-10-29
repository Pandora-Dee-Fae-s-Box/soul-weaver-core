use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::error::{Error, Result};

/// A generic vault trait for storing durable fragments.
pub trait Vault: Send + Sync {
    fn list(&self) -> Result<Vec<String>>;
    fn get(&self, path: &str) -> Result<VaultEntry>;
    fn put(&mut self, entry: VaultEntry) -> Result<()>;
    fn remove(&mut self, path: &str) -> Result<()>;
}

/// A single entry in a vault.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultEntry {
    pub path: String,
    pub content: Vec<u8>,
    pub version: String,
    pub modified: DateTime<Utc>,
}

/// In-memory vault implementation for tests and bootstrapping.
#[derive(Default)]
pub struct InMemoryVault {
    entries: HashMap<String, VaultEntry>,
}

impl InMemoryVault {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Vault for InMemoryVault {
    fn list(&self) -> Result<Vec<String>> {
        let mut out: Vec<String> = self.entries.keys().cloned().collect();
        out.sort();
        Ok(out)
    }

    fn get(&self, path: &str) -> Result<VaultEntry> {
        self.entries
            .get(path)
            .cloned()
            .ok_or_else(|| Error::VaultEntryNotFound(path.to_string()))
    }

    fn put(&mut self, entry: VaultEntry) -> Result<()> {
        self.entries.insert(entry.path.clone(), entry);
        Ok(())
    }

    fn remove(&mut self, path: &str) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }
}
