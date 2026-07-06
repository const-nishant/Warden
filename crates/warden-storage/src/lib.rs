//! Encrypted local storage backed by SQLite.
//!
//! Manages contacts, messages, outbox entries, and groups in a local
//! SQLite database (rusqlite, bundled). All store traits (`ContactStore`,
//! [`MessageStore`], [`OutboxStore`], [`GroupStore`]) are implemented
//! on [`Database`].

pub mod contacts;
pub mod database;
pub mod error;
pub mod groups;
pub mod messages;
pub mod outbox;

pub use contacts::{Contact, ContactStore};
pub use database::Database;
pub use error::StorageError;
pub use groups::{Group, GroupMember, GroupStore};
pub use messages::{MessageStore, StoredMessage};
pub use outbox::{OutboxEntry, OutboxStore};
