
pub mod models {
    include!(concat!(env!("OUT_DIR"), "/model.ciphertext.rs"));
}

// Re-export types for convenient access
pub use models::{Ciphertext, CiphertextMetadata, EncodingMetadata, Encoding};