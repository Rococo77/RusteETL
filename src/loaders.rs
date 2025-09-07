//! Module des loaders (destinations). Exemple: CSV, PostgreSQL, Parquet
//
// Exemple: CSV, PostgreSQL, Parquet
pub trait Loader {
    /// Charge les données dans la destination
    fn load(&self, data: Vec<Vec<String>>) -> Result<(), Box<dyn std::error::Error>>;
}

/// Exemple de loader CSV minimal
pub struct CsvLoader {
    pub path: String,
}

impl Loader for CsvLoader {
    fn load(&self, data: Vec<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implémenter l'écriture CSV réelle
        println!("Écriture dans {}: {:?}", self.path, data);
        Ok(())
    }
}
