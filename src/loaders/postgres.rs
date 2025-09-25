//! Loader PostgreSQL (stub)

use crate::error::EtlError;

pub struct PostgresLoader;

impl PostgresLoader {
    pub fn load(&self, _data: Vec<Vec<String>>) -> Result<(), EtlError> {
        // TODO: Implémentation réelle
        Err(EtlError::NotImplemented)
    }
}
