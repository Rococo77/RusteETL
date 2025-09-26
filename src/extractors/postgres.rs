//! Extracteur PostgreSQL (stub)

use crate::error::EtlError;

#[derive(Clone)]
pub struct PostgresExtractor;

impl PostgresExtractor {
    pub fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        // TODO: Implémentation réelle
        Err(EtlError::NotImplemented)
    }

    pub async fn extract_async(&self) -> Result<Vec<Vec<String>>, EtlError> {
        let me = self.clone();
        let res = tokio::task::spawn_blocking(move || me.extract())
            .await
            .map_err(|e| EtlError::Other(format!("Task join error: {}", e)))??;
        Ok(res)
    }
}
