//! Module des extracteurs de données (sources). Exemple: CSV, PostgreSQL, HTTP, etc.

/// Exemple: CSV, PostgreSQL, HTTP, etc.
pub trait Extractor {
    /// Extrait les données et retourne un vecteur de lignes (chaque ligne est un vecteur de chaînes)
    fn extract(&self) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>>;
}

/// Exemple d'extracteur CSV minimal
#[allow(dead_code)]
pub struct CsvExtractor {
    pub path: String,
}

impl Extractor for CsvExtractor {
    fn extract(&self) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        // TODO: Implémenter la lecture CSV réelle
        Ok(vec![vec!["col1".to_string(), "col2".to_string()]])
    }
}
