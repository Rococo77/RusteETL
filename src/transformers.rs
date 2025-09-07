//! Module des transformations de données. Exemple: filtre, mapping, agrégation
//
// Exemple: filtre, mapping, agrégation
pub trait Transformer {
    /// Transforme les données (vecteur de lignes)
    fn transform(&self, data: Vec<Vec<String>>) -> Vec<Vec<String>>;
}

/// Exemple de transformation: majuscule sur la première colonne
pub struct UppercaseTransformer;

impl Transformer for UppercaseTransformer {
    fn transform(&self, mut data: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for row in &mut data {
            if let Some(first) = row.get_mut(0) {
                *first = first.to_uppercase();
            }
        }
        data
    }
}
