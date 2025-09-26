pub mod filter;
pub mod uppercase;
pub mod map;

// Re-export to preserve previous public API
pub use crate::pipeline::Transformer;
pub use uppercase::UppercaseTransformer;
pub use map::MapTransformer;
