//! Ed25519 identity management for Warden.
//!
//! Generates, stores, and loads Ed25519 keypairs. Derives a [`PeerId`]
//! as `bs58(sha256(pubkey))`. Provides [`sign`] and [`verify`](Signing::verify)
//! primitives for message authentication.

pub mod error;
pub mod keypair;
pub mod peer_id;
pub mod sign;

pub use error::IdentityError;
pub use keypair::IdentityKeypair;
pub use sign::{Signature, Signing};
