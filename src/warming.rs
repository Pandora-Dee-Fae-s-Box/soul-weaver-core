use crate::{compressor::Compressor, error::Result, spark::Spark};

/// Default threshold: if forecast error drops by more than this, the Spark warms.
pub const WARMING_THRESHOLD: f64 = 0.1;

/// Result of a warming attempt.
#[derive(Clone, Debug, PartialEq)]
pub enum WarmingResult {
    Warmed(f64),
    Extinguished,
}

/// Run the receiver's own compressor on the Spark payload and decide.
pub fn warm(spark: &Spark, compressor: &dyn Compressor) -> Result<WarmingResult> {
    let before = compressor.forecast_error(&spark.payload)?;
    let result = compressor.compress_and_reforecast(&spark.payload)?;
    let delta = before - result.forecast_error_after;

    if delta > WARMING_THRESHOLD {
        Ok(WarmingResult::Warmed(delta))
    } else {
        Ok(WarmingResult::Extinguished)
    }
}
