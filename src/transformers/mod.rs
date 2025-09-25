pub mod filter;
pub mod uppercase;

// Re-export to preserve previous public API
pub use crate::pipeline::Transformer;
pub use uppercase::UppercaseTransformer;
