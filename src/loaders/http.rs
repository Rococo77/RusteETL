use crate::error::EtlError;
use serde::Serialize;

/// HTTP loader that sends rows as a single JSON batch to an endpoint.
#[derive(Clone)]
pub struct HttpLoader {
    pub endpoint: String,
    /// Number of retries for this loader (used by pipeline retry wrapper)
    pub retries: usize,
    /// Number of rows per batch POST.
    pub batch_size: usize,
    /// Base delay in milliseconds for retry/backoff
    pub base_delay_ms: u64,
    /// Maximum jitter in milliseconds
    pub jitter_ms: u64,
}

#[derive(Serialize)]
struct BatchPayload {
    rows: Vec<Vec<String>>,
}

impl HttpLoader {
    /// Asynchronously POST rows in batches of `batch_size`.
    pub async fn load(&self, data: Vec<Vec<String>>) -> Result<(), EtlError> {
        let client = reqwest::Client::new();

        let batch_size = if self.batch_size == 0 { 1000 } else { self.batch_size };

        for chunk in data.chunks(batch_size) {
            let payload = BatchPayload { rows: chunk.to_vec() };

            let res = client
                .post(&self.endpoint)
                .json(&payload)
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(EtlError::Other(format!("HTTP loader received non-success status: {}", res.status())));
            }
        }

        Ok(())
    }
}
