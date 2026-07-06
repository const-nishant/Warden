use crate::StorageError;

#[derive(Debug, Clone)]
pub struct OutboxEntry {
    pub id: String,
    pub target_peer_id: String,
    pub frame_bytes: Vec<u8>,
    pub created_at_ms: i64,
    pub retry_count: u32,
    pub last_attempt_ms: Option<i64>,
    pub delivered: bool,
}

pub trait OutboxStore {
    fn enqueue(&self, entry: OutboxEntry) -> Result<(), StorageError>;
    fn pending_for_peer(&self, peer_id: &str) -> Result<Vec<OutboxEntry>, StorageError>;
    fn all_pending(&self) -> Result<Vec<OutboxEntry>, StorageError>;
    fn mark_delivered_outbox(&self, id: &str) -> Result<(), StorageError>;
    fn increment_retry(&self, id: &str, now_ms: i64) -> Result<(), StorageError>;
    fn remove_outbox_entry(&self, id: &str) -> Result<(), StorageError>;
}
