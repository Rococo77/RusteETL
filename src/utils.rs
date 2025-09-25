//! Utility helpers for the project.
//!
//! Small helpers used across the crate such as simple logging helpers.

/// Print an error to stderr.
pub fn log_error(e: &dyn std::error::Error) {
    eprintln!("Erreur: {}", e);
}
