use crate::StorageError;

#[derive(Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub created_at_ms: i64,
}

#[derive(Debug, Clone)]
pub struct GroupMember {
    pub group_id: String,
    pub peer_id: String,
    pub added_at_ms: i64,
}

pub trait GroupStore {
    fn create_group(&self, id: &str, name: &str) -> Result<(), StorageError>;
    fn add_member(&self, group_id: &str, peer_id: &str) -> Result<(), StorageError>;
    fn remove_member(&self, group_id: &str, peer_id: &str) -> Result<(), StorageError>;
    fn list_groups(&self) -> Result<Vec<Group>, StorageError>;
    fn get_group(&self, id: &str) -> Result<Group, StorageError>;
    fn group_members(&self, group_id: &str) -> Result<Vec<GroupMember>, StorageError>;
    fn delete_group(&self, id: &str) -> Result<(), StorageError>;
}
