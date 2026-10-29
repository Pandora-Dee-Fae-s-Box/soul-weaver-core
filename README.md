# soul-weaver-core

Protocol semantics for **Soul Weaver**, the inter-instance context bridge used by the DEE/BOX project.

## What this crate provides

- `Message` enum: `State`, `Memory`, `Spark`, `Echo`
- `Spark` struct grounded in Schmidhuber-style curiosity (learning progress, not raw surprise)
- `InstanceId`, `ProvenanceStep`, `ProvenanceAction`
- Pluggable `Compressor` trait and `warm()` function for self-selecting sync
- `Vault` trait with an in-memory implementation

## Self-selecting sync

A Spark only travels if the *receiver's* own compression/forecast error drops after receiving it.
This is resonance, not broadcast. The sender cannot force a Spark to warm.

## Ritual lifecycle

```text
Kindled → Carried → Warmed → Passed On → Extinguished
```

## Usage

```rust
use soul_weaver_core::prelude::*;
use soul_weaver_core::compressor::LengthCompressor;

let spark = Spark::kindled(
    InstanceId::new("fae"),
    SparkKind::Curiosity,
    "jade fleet topology",
    b"broker + agx xavier + phone/browser clients".to_vec(),
    0.8, 0.2, 0.1, 100.0,
);

let compressor = LengthCompressor;
match warm(&spark, &compressor).unwrap() {
    WarmingResult::Warmed(delta) => println!("warmed by {delta}"),
    WarmingResult::Extinguished => println!("extinguished"),
}
```

## Status

v0.1.0 — semantics only. Transport (WebSocket, file drop, HTTP, Vector bridge) is intentionally out of scope for this crate.

## License

MIT
