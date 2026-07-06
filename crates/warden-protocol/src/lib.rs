//! Protobuf message framing and outbox queue.
//!
//! [`MessageFrame`] encodes/decodes the `ChatFrame` protobuf with
//! message_id, sender, timestamp, ciphertext, and signature.
//! [`Outbox`] is an in-memory store-and-forward queue for offline delivery.

pub mod error;
pub mod frame;
pub mod outbox;
pub mod session;

pub use error::ProtocolError;
pub use frame::MessageFrame;
pub use outbox::Outbox;
pub use session::{ProtocolSession, SessionState};
