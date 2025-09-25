//! Extracteur PostgreSQL (stub)

use crate::error::EtlError;

pub struct PostgresExtractor;

impl PostgresExtractor {
    pub fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        // TODO: Implémentation réelle
        Err(EtlError::NotImplemented)
    }
}
