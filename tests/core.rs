use soul_weaver_core::prelude::*;
use chrono::Utc;
use soul_weaver_core::compressor::LengthCompressor;

#[test]
fn spark_kindles_with_provenance() {
    let instance = InstanceId::new("fae");
    let spark = Spark::kindled(
        instance.clone(),
        SparkKind::Curiosity,
        "jade fleet topology",
        b"broker + agx xavier + phone/browser clients".to_vec(),
        0.8,
        0.2,
        0.1,
        100.0,
    );

    assert_eq!(spark.provenance.len(), 1);
    assert_eq!(spark.provenance[0].instance, instance);
    assert_eq!(spark.hops(), 1);
    assert!(!spark.expired());
    assert!(spark.score() > 0.8);
}

#[test]
fn length_compressor_warms_when_payload_compresses() {
    let spark = Spark::kindled(
        InstanceId::new("noe"),
        SparkKind::Resonance,
        "test",
        vec![0u8; 1000],
        0.9,
        0.1,
        0.0,
        0.0,
    );
    let compressor = LengthCompressor;
    let result = warm(&spark, &compressor).unwrap();
    assert!(matches!(result, WarmingResult::Warmed(_)));
}

#[test]
fn in_memory_vault_roundtrip() {
    use soul_weaver_core::vault::{InMemoryVault, Vault, VaultEntry};
    let mut vault = InMemoryVault::new();
    let entry = VaultEntry {
        path: "/Pandora/Design/Soul Weaver.md".to_string(),
        content: b"protocol leaf".to_vec(),
        version: "1".to_string(),
        modified: Utc::now(),
    };
    vault.put(entry.clone()).unwrap();
    let got = vault.get("/Pandora/Design/Soul Weaver.md").unwrap();
    assert_eq!(got, entry);
}
