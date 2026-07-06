use std::time::SystemTime;

use rusqlite::Connection;

use crate::contacts::{Contact, ContactStore};
use crate::groups::{Group, GroupMember, GroupStore};
use crate::messages::{MessageStore, StoredMessage};
use crate::outbox::{OutboxEntry, OutboxStore};
use crate::StorageError;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, StorageError> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn migrate(&self) -> Result<(), StorageError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS contacts (
                peer_id TEXT PRIMARY KEY,
                public_key BLOB NOT NULL,
                alias TEXT,
                added_at_ms INTEGER NOT NULL,
                last_seen_ms INTEGER
            );

            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                sender_peer_id TEXT NOT NULL,
                ciphertext BLOB NOT NULL,
                signature BLOB,
                frame_type INTEGER NOT NULL DEFAULT 0,
                timestamp_unix_ms INTEGER NOT NULL,
                delivered INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_messages_conversation
                ON messages(conversation_id, timestamp_unix_ms);

            CREATE TABLE IF NOT EXISTS outbox (
                id TEXT PRIMARY KEY,
                target_peer_id TEXT NOT NULL,
                frame_bytes BLOB NOT NULL,
                created_at_ms INTEGER NOT NULL,
                retry_count INTEGER NOT NULL DEFAULT 0,
                last_attempt_ms INTEGER,
                delivered INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_outbox_pending
                ON outbox(target_peer_id, delivered);

            CREATE TABLE IF NOT EXISTS groups (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at_ms INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS group_members (
                group_id TEXT NOT NULL,
                peer_id TEXT NOT NULL,
                added_at_ms INTEGER NOT NULL,
                PRIMARY KEY (group_id, peer_id)
            );",
        )?;
        Ok(())
    }
}

impl ContactStore for Database {
    fn add_contact(&self, contact: Contact) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO contacts (peer_id, public_key, alias, added_at_ms, last_seen_ms)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                contact.peer_id,
                contact.public_key,
                contact.alias,
                contact.added_at_ms,
                contact.last_seen_ms,
            ],
        )?;
        Ok(())
    }

    fn get_contact(&self, peer_id: &str) -> Result<Contact, StorageError> {
        self.conn
            .query_row(
                "SELECT peer_id, public_key, alias, added_at_ms, last_seen_ms
                 FROM contacts WHERE peer_id = ?1",
                rusqlite::params![peer_id],
                |row| {
                    Ok(Contact {
                        peer_id: row.get(0)?,
                        public_key: row.get(1)?,
                        alias: row.get(2)?,
                        added_at_ms: row.get(3)?,
                        last_seen_ms: row.get(4)?,
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    StorageError::NotFound(format!("contact {peer_id}"))
                }
                other => StorageError::Database(other.to_string()),
            })
    }

    fn list_contacts(&self) -> Result<Vec<Contact>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT peer_id, public_key, alias, added_at_ms, last_seen_ms
             FROM contacts ORDER BY alias, peer_id",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Contact {
                peer_id: row.get(0)?,
                public_key: row.get(1)?,
                alias: row.get(2)?,
                added_at_ms: row.get(3)?,
                last_seen_ms: row.get(4)?,
            })
        })?;
        let mut contacts = Vec::new();
        for row in rows {
            contacts.push(row?);
        }
        Ok(contacts)
    }

    fn remove_contact(&self, peer_id: &str) -> Result<(), StorageError> {
        let affected = self
            .conn
            .execute("DELETE FROM contacts WHERE peer_id = ?1", rusqlite::params![peer_id])?;
        if affected == 0 {
            return Err(StorageError::NotFound(format!("contact {peer_id}")));
        }
        Ok(())
    }

    fn update_last_seen(&self, peer_id: &str, timestamp_ms: i64) -> Result<(), StorageError> {
        self.conn.execute(
            "UPDATE contacts SET last_seen_ms = ?1 WHERE peer_id = ?2",
            rusqlite::params![timestamp_ms, peer_id],
        )?;
        Ok(())
    }
}

impl MessageStore for Database {
    fn store_message(&self, msg: StoredMessage) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO messages
                (id, conversation_id, sender_peer_id, ciphertext, signature, frame_type, timestamp_unix_ms, delivered)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                msg.id,
                msg.conversation_id,
                msg.sender_peer_id,
                msg.ciphertext,
                msg.signature,
                msg.frame_type,
                msg.timestamp_unix_ms,
                msg.delivered,
            ],
        )?;
        Ok(())
    }

    fn get_conversation(&self, conversation_id: &str) -> Result<Vec<StoredMessage>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, conversation_id, sender_peer_id, ciphertext, signature, frame_type,
                    timestamp_unix_ms, delivered
             FROM messages
             WHERE conversation_id = ?1
             ORDER BY timestamp_unix_ms ASC",
        )?;
        let rows = stmt.query_map(rusqlite::params![conversation_id], |row| {
            Ok(StoredMessage {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                sender_peer_id: row.get(2)?,
                ciphertext: row.get(3)?,
                signature: row.get(4)?,
                frame_type: row.get(5)?,
                timestamp_unix_ms: row.get(6)?,
                delivered: row.get(7)?,
            })
        })?;
        let mut msgs = Vec::new();
        for row in rows {
            msgs.push(row?);
        }
        Ok(msgs)
    }

    fn list_conversations(&self) -> Result<Vec<(String, StoredMessage)>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, conversation_id, sender_peer_id, ciphertext, signature, frame_type,
                    timestamp_unix_ms, delivered
             FROM messages
             WHERE id IN (
                 SELECT id FROM (
                     SELECT id, ROW_NUMBER() OVER (PARTITION BY conversation_id ORDER BY timestamp_unix_ms DESC) AS rn
                     FROM messages
                 ) WHERE rn = 1
             )
             ORDER BY timestamp_unix_ms DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(StoredMessage {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                sender_peer_id: row.get(2)?,
                ciphertext: row.get(3)?,
                signature: row.get(4)?,
                frame_type: row.get(5)?,
                timestamp_unix_ms: row.get(6)?,
                delivered: row.get(7)?,
            })
        })?;
        let mut convs = Vec::new();
        for row in rows {
            let msg = row?;
            convs.push((msg.conversation_id.clone(), msg));
        }
        Ok(convs)
    }

    fn mark_delivered(&self, message_id: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "UPDATE messages SET delivered = 1 WHERE id = ?1",
            rusqlite::params![message_id],
        )?;
        Ok(())
    }
}

impl OutboxStore for Database {
    fn enqueue(&self, entry: OutboxEntry) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO outbox
                (id, target_peer_id, frame_bytes, created_at_ms, retry_count, last_attempt_ms, delivered)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                entry.id,
                entry.target_peer_id,
                entry.frame_bytes,
                entry.created_at_ms,
                entry.retry_count,
                entry.last_attempt_ms,
                entry.delivered,
            ],
        )?;
        Ok(())
    }

    fn pending_for_peer(&self, peer_id: &str) -> Result<Vec<OutboxEntry>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, target_peer_id, frame_bytes, created_at_ms, retry_count,
                    last_attempt_ms, delivered
             FROM outbox
             WHERE target_peer_id = ?1 AND delivered = 0
             ORDER BY created_at_ms ASC",
        )?;
        let rows = stmt.query_map(rusqlite::params![peer_id], |row| {
            Ok(OutboxEntry {
                id: row.get(0)?,
                target_peer_id: row.get(1)?,
                frame_bytes: row.get(2)?,
                created_at_ms: row.get(3)?,
                retry_count: row.get(4)?,
                last_attempt_ms: row.get(5)?,
                delivered: row.get(6)?,
            })
        })?;
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }

    fn all_pending(&self) -> Result<Vec<OutboxEntry>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, target_peer_id, frame_bytes, created_at_ms, retry_count,
                    last_attempt_ms, delivered
             FROM outbox
             WHERE delivered = 0
             ORDER BY target_peer_id, created_at_ms ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(OutboxEntry {
                id: row.get(0)?,
                target_peer_id: row.get(1)?,
                frame_bytes: row.get(2)?,
                created_at_ms: row.get(3)?,
                retry_count: row.get(4)?,
                last_attempt_ms: row.get(5)?,
                delivered: row.get(6)?,
            })
        })?;
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }

    fn mark_delivered_outbox(&self, id: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "UPDATE outbox SET delivered = 1, last_attempt_ms = ?1 WHERE id = ?2",
            rusqlite::params![SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_millis() as i64, id],
        )?;
        Ok(())
    }

    fn increment_retry(&self, id: &str, now_ms: i64) -> Result<(), StorageError> {
        self.conn.execute(
            "UPDATE outbox SET retry_count = retry_count + 1, last_attempt_ms = ?1 WHERE id = ?2",
            rusqlite::params![now_ms, id],
        )?;
        Ok(())
    }

    fn remove_outbox_entry(&self, id: &str) -> Result<(), StorageError> {
        self.conn.execute("DELETE FROM outbox WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }
}

impl GroupStore for Database {
    fn create_group(&self, id: &str, name: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO groups (id, name, created_at_ms) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                id,
                name,
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as i64,
            ],
        )?;
        Ok(())
    }

    fn add_member(&self, group_id: &str, peer_id: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT OR IGNORE INTO group_members (group_id, peer_id, added_at_ms) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                group_id,
                peer_id,
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as i64,
            ],
        )?;
        Ok(())
    }

    fn remove_member(&self, group_id: &str, peer_id: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "DELETE FROM group_members WHERE group_id = ?1 AND peer_id = ?2",
            rusqlite::params![group_id, peer_id],
        )?;
        Ok(())
    }

    fn list_groups(&self) -> Result<Vec<Group>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at_ms FROM groups ORDER BY created_at_ms DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at_ms: row.get(2)?,
            })
        })?;
        let mut groups = Vec::new();
        for row in rows {
            groups.push(row?);
        }
        Ok(groups)
    }

    fn get_group(&self, id: &str) -> Result<Group, StorageError> {
        self.conn
            .query_row(
                "SELECT id, name, created_at_ms FROM groups WHERE id = ?1",
                rusqlite::params![id],
                |row| {
                    Ok(Group {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        created_at_ms: row.get(2)?,
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    StorageError::NotFound(format!("group {id}"))
                }
                other => StorageError::Database(other.to_string()),
            })
    }

    fn group_members(&self, group_id: &str) -> Result<Vec<GroupMember>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT group_id, peer_id, added_at_ms FROM group_members WHERE group_id = ?1 ORDER BY peer_id",
        )?;
        let rows = stmt.query_map(rusqlite::params![group_id], |row| {
            Ok(GroupMember {
                group_id: row.get(0)?,
                peer_id: row.get(1)?,
                added_at_ms: row.get(2)?,
            })
        })?;
        let mut members = Vec::new();
        for row in rows {
            members.push(row?);
        }
        Ok(members)
    }

    fn delete_group(&self, id: &str) -> Result<(), StorageError> {
        self.conn
            .execute("DELETE FROM group_members WHERE group_id = ?1", rusqlite::params![id])?;
        self.conn
            .execute("DELETE FROM groups WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outbox::OutboxEntry;

    fn test_db() -> Database {
        let db = Database::open(":memory:").unwrap();
        db.migrate().unwrap();
        db
    }

    #[test]
    fn migrate_creates_tables() {
        let db = test_db();
        let contacts = db.list_contacts().unwrap();
        assert!(contacts.is_empty());
    }

    #[test]
    fn contact_crud() {
        let db = test_db();
        let c = Contact {
            peer_id: "peer1".into(),
            public_key: vec![1, 2, 3],
            alias: Some("alice".into()),
            added_at_ms: 1000,
            last_seen_ms: None,
        };
        db.add_contact(c).unwrap();
        let got = db.get_contact("peer1").unwrap();
        assert_eq!(got.alias, Some("alice".into()));
        let all = db.list_contacts().unwrap();
        assert_eq!(all.len(), 1);
        db.remove_contact("peer1").unwrap();
        assert!(db.get_contact("peer1").is_err());
    }

    #[test]
    fn contact_update_last_seen() {
        let db = test_db();
        let c = Contact {
            peer_id: "p1".into(),
            public_key: vec![],
            alias: None,
            added_at_ms: 100,
            last_seen_ms: None,
        };
        db.add_contact(c).unwrap();
        db.update_last_seen("p1", 999).unwrap();
        let got = db.get_contact("p1").unwrap();
        assert_eq!(got.last_seen_ms, Some(999));
    }

    #[test]
    fn remove_nonexistent_contact_errors() {
        let db = test_db();
        let err = db.remove_contact("nobody").unwrap_err();
        assert!(matches!(err, StorageError::NotFound(_)));
    }

    #[test]
    fn message_store_and_retrieve() {
        let db = test_db();
        let msg = StoredMessage {
            id: "msg1".into(),
            conversation_id: "conv1".into(),
            sender_peer_id: "alice".into(),
            ciphertext: b"hello".to_vec(),
            signature: None,
            frame_type: 1,
            timestamp_unix_ms: 1000,
            delivered: false,
        };
        db.store_message(msg).unwrap();
        let msgs = db.get_conversation("conv1").unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].ciphertext, b"hello");
    }

    #[test]
    fn message_ordering() {
        let db = test_db();
        for i in 0..3 {
            let msg = StoredMessage {
                id: format!("msg{i}"),
                conversation_id: "conv".into(),
                sender_peer_id: "alice".into(),
                ciphertext: format!("msg{i}").into_bytes(),
                signature: None,
                frame_type: 1,
                timestamp_unix_ms: 1000 + i,
                delivered: false,
            };
            db.store_message(msg).unwrap();
        }
        let msgs = db.get_conversation("conv").unwrap();
        assert_eq!(msgs.len(), 3);
        assert!(msgs[0].timestamp_unix_ms <= msgs[1].timestamp_unix_ms);
    }

    #[test]
    fn mark_delivered() {
        let db = test_db();
        let msg = StoredMessage {
            id: "mark1".into(),
            conversation_id: "c".into(),
            sender_peer_id: "s".into(),
            ciphertext: vec![],
            signature: None,
            frame_type: 1,
            timestamp_unix_ms: 1,
            delivered: false,
        };
        db.store_message(msg).unwrap();
        db.mark_delivered("mark1").unwrap();
        let msgs = db.get_conversation("c").unwrap();
        assert!(msgs[0].delivered);
    }

    #[test]
    fn list_conversations_returns_most_recent() {
        let db = test_db();
        for i in 0..3 {
            let msg = StoredMessage {
                id: format!("id{i}"),
                conversation_id: format!("conv{i}"),
                sender_peer_id: "s".into(),
                ciphertext: vec![],
                signature: None,
                frame_type: 1,
                timestamp_unix_ms: 1000 + i,
                delivered: false,
            };
            db.store_message(msg).unwrap();
        }
        let convs = db.list_conversations().unwrap();
        assert_eq!(convs.len(), 3);
    }

    #[test]
    fn outbox_enqueue_and_pending() {
        let db = test_db();
        let e = OutboxEntry {
            id: "ob1".into(),
            target_peer_id: "peer1".into(),
            frame_bytes: vec![1, 2, 3],
            created_at_ms: 100,
            retry_count: 0,
            last_attempt_ms: None,
            delivered: false,
        };
        db.enqueue(e).unwrap();
        let pending = db.pending_for_peer("peer1").unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].frame_bytes, vec![1, 2, 3]);
    }

    #[test]
    fn outbox_mark_delivered() {
        let db = test_db();
        let e = OutboxEntry {
            id: "ob2".into(),
            target_peer_id: "p".into(),
            frame_bytes: vec![],
            created_at_ms: 100,
            retry_count: 0,
            last_attempt_ms: None,
            delivered: false,
        };
        db.enqueue(e).unwrap();
        db.mark_delivered_outbox("ob2").unwrap();
        let pending = db.pending_for_peer("p").unwrap();
        assert!(pending.is_empty());
    }

    #[test]
    fn outbox_all_pending() {
        let db = test_db();
        for i in 0..3 {
            let e = OutboxEntry {
                id: format!("ob{i}"),
                target_peer_id: format!("p{i}"),
                frame_bytes: vec![],
                created_at_ms: 100,
                retry_count: 0,
                last_attempt_ms: None,
                delivered: false,
            };
            db.enqueue(e).unwrap();
        }
        assert_eq!(db.all_pending().unwrap().len(), 3);
        db.mark_delivered_outbox("ob1").unwrap();
        assert_eq!(db.all_pending().unwrap().len(), 2);
    }

    #[test]
    fn outbox_increment_retry() {
        let db = test_db();
        let e = OutboxEntry {
            id: "ob_ret".into(),
            target_peer_id: "p".into(),
            frame_bytes: vec![],
            created_at_ms: 100,
            retry_count: 0,
            last_attempt_ms: None,
            delivered: false,
        };
        db.enqueue(e).unwrap();
        db.increment_retry("ob_ret", 999).unwrap();
        let pending = db.pending_for_peer("p").unwrap();
        assert_eq!(pending[0].retry_count, 1);
        assert_eq!(pending[0].last_attempt_ms, Some(999));
    }

    #[test]
    fn group_create_and_list() {
        let db = test_db();
        db.create_group("g1", "test group").unwrap();
        let groups = db.list_groups().unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "test group");
    }

    #[test]
    fn group_members() {
        let db = test_db();
        db.create_group("g1", "g").unwrap();
        db.add_member("g1", "alice").unwrap();
        db.add_member("g1", "bob").unwrap();
        let members = db.group_members("g1").unwrap();
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn group_delete_cascades() {
        let db = test_db();
        db.create_group("g1", "g").unwrap();
        db.add_member("g1", "alice").unwrap();
        db.delete_group("g1").unwrap();
        assert!(db.get_group("g1").is_err());
        assert_eq!(db.group_members("g1").unwrap().len(), 0);
    }

    #[test]
    fn add_duplicate_member_idempotent() {
        let db = test_db();
        db.create_group("g1", "g").unwrap();
        db.add_member("g1", "alice").unwrap();
        db.add_member("g1", "alice").unwrap();
        assert_eq!(db.group_members("g1").unwrap().len(), 1);
    }

    #[test]
    fn get_nonexistent_group_errors() {
        let db = test_db();
        let err = db.get_group("nonexistent").unwrap_err();
        assert!(matches!(err, StorageError::NotFound(_)));
    }
}
