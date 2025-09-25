pub mod uppercase;
pub mod filter;

// Re-export to preserve previous public API
pub use crate::pipeline::Transformer;
pub use uppercase::UppercaseTransformer;
