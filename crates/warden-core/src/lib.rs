//! Core types and shared primitives for Warden.
//!
//! Provides [`PeerId`], [`MessageId`], [`TimestampMillis`], and the
//! generated Protobuf types (`warden.v1`). All other crates depend on this one.

pub mod error;
pub mod types;

pub use error::CoreError;
pub use types::proto;
pub use types::*;
