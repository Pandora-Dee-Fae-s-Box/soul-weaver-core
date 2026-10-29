use crate::error::Result;

/// A pluggable compression/forecast model used by each instance.
pub trait Compressor: Send + Sync {
    /// Forecast error before seeing the payload.
    fn forecast_error(&self, payload: &[u8]) -> Result<f64>;

    /// Compress the payload and return the new forecast error.
    fn compress_and_reforecast(&self, payload: &[u8]) -> Result<CompressorResult>;
}

/// Result of compressing a payload.
#[derive(Clone, Debug, PartialEq)]
pub struct CompressorResult {
    pub forecast_error_after: f64,
    pub compression_ratio: f64,
}

/// A trivial compressor for tests: gzip-ish length estimate.
#[derive(Default)]
pub struct LengthCompressor;

impl Compressor for LengthCompressor {
    fn forecast_error(&self, payload: &[u8]) -> Result<f64> {
        // Naive: longer payloads are harder to predict.
        Ok(payload.len() as f64)
    }

    fn compress_and_reforecast(&self, payload: &[u8]) -> Result<CompressorResult> {
        let after = (payload.len() as f64) * 0.7;
        Ok(CompressorResult {
            forecast_error_after: after,
            compression_ratio: 0.7,
        })
    }
}
