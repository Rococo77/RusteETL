//! Module utilitaire: gestion des erreurs, logging, etc.

/// Affiche une erreur sur stderr
pub fn log_error(e: &dyn std::error::Error) {
    eprintln!("Erreur: {}", e);
}
