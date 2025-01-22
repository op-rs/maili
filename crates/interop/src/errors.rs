//! Error types for interop.

use thiserror::Error;

/// An error type for the [SuperRoot] struct's serialization and deserialization.
///
/// [SuperRoot]: crate::SuperRoot
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SuperRootError {
    /// Invalid super root version byte
    #[error("Invalid super root version byte")]
    InvalidVersionByte,
    /// Unexpected encoded super root length
    #[error("Unexpected encoded super root length")]
    UnexpectedLength,
}

/// A [Result] alias for the [SuperRootError] type.
pub type SuperRootResult<T> = core::result::Result<T, SuperRootError>;
