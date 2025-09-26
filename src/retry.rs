use rand::Rng;
use std::time::Duration;

/// Retry an async operation with exponential backoff and jitter.
/// `op` is an async closure returning Result<T, E>. The closure will be invoked
/// up to `max_attempts` times. Base delay is `base_delay`.
pub async fn retry_async<F, Fut, T, E>(mut op: F, max_attempts: usize, base_delay: Duration, jitter_ms: u64) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    let mut rng = rand::thread_rng();

    loop {
        attempt += 1;
        match op().await {
            Ok(v) => return Ok(v),
            Err(e) if attempt >= max_attempts => return Err(e),
            Err(_) => {
                // exponential backoff with jitter
                let exp = 2_u64.pow((attempt - 1) as u32);
                let jitter_rand: u64 = if jitter_ms == 0 { 0 } else { rng.gen_range(0..=jitter_ms) };
                // base_delay * exp where base_delay is Duration: multiply its millis
                let base_ms = base_delay.as_millis() as u64;
                let delay_ms = base_ms.saturating_mul(exp) + jitter_rand;
                let delay = Duration::from_millis(delay_ms);
                tokio::time::sleep(delay).await;
            }
        }
    }
}
