use crate::StorageError;

#[derive(Debug, Clone)]
pub struct StoredMessage {
    pub id: String,
    pub conversation_id: String,
    pub sender_peer_id: String,
    pub ciphertext: Vec<u8>,
    pub signature: Option<Vec<u8>>,
    pub frame_type: i32,
    pub timestamp_unix_ms: i64,
    pub delivered: bool,
}

pub trait MessageStore {
    fn store_message(&self, msg: StoredMessage) -> Result<(), StorageError>;
    fn get_conversation(&self, conversation_id: &str) -> Result<Vec<StoredMessage>, StorageError>;
    fn list_conversations(&self) -> Result<Vec<(String, StoredMessage)>, StorageError>;
    fn mark_delivered(&self, message_id: &str) -> Result<(), StorageError>;
}
