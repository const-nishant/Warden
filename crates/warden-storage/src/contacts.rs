use crate::StorageError;

#[derive(Debug, Clone)]
pub struct Contact {
    pub peer_id: String,
    pub public_key: Vec<u8>,
    pub alias: Option<String>,
    pub added_at_ms: i64,
    pub last_seen_ms: Option<i64>,
}

pub trait ContactStore {
    fn add_contact(&self, contact: Contact) -> Result<(), StorageError>;
    fn get_contact(&self, peer_id: &str) -> Result<Contact, StorageError>;
    fn list_contacts(&self) -> Result<Vec<Contact>, StorageError>;
    fn remove_contact(&self, peer_id: &str) -> Result<(), StorageError>;
    fn update_last_seen(&self, peer_id: &str, timestamp_ms: i64) -> Result<(), StorageError>;
}
