use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxEntry {
    pub peer_id: String,
    pub message_id: String,
    pub frame_bytes: Vec<u8>,
    pub created_at_ms: i64,
    pub retry_count: u32,
}

pub struct Outbox {
    entries: Vec<OutboxEntry>,
}

impl Outbox {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, entry: OutboxEntry) {
        self.entries.push(entry);
    }

    pub fn pending(&self) -> Vec<&OutboxEntry> {
        self.entries.iter().collect()
    }

    pub fn remove(&mut self, message_id: &str) {
        self.entries.retain(|e| e.message_id != message_id);
    }
}

impl Default for Outbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outbox_new_is_empty() {
        let ob = Outbox::new();
        assert!(ob.pending().is_empty());
    }

    #[test]
    fn enqueue_and_pending() {
        let mut ob = Outbox::new();
        let entry = OutboxEntry {
            peer_id: "peer1".into(),
            message_id: "mid1".into(),
            frame_bytes: vec![1, 2, 3],
            created_at_ms: 100,
            retry_count: 0,
        };
        ob.enqueue(entry);
        assert_eq!(ob.pending().len(), 1);
        assert_eq!(ob.pending()[0].peer_id, "peer1");
    }

    #[test]
    fn remove_entry() {
        let mut ob = Outbox::new();
        ob.enqueue(OutboxEntry {
            peer_id: "p".into(),
            message_id: "m1".into(),
            frame_bytes: vec![],
            created_at_ms: 0,
            retry_count: 0,
        });
        ob.enqueue(OutboxEntry {
            peer_id: "p".into(),
            message_id: "m2".into(),
            frame_bytes: vec![],
            created_at_ms: 0,
            retry_count: 0,
        });
        ob.remove("m1");
        assert_eq!(ob.pending().len(), 1);
        assert_eq!(ob.pending()[0].message_id, "m2");
    }

    #[test]
    fn remove_nonexistent_noop() {
        let mut ob = Outbox::new();
        ob.enqueue(OutboxEntry {
            peer_id: "p".into(),
            message_id: "m1".into(),
            frame_bytes: vec![],
            created_at_ms: 0,
            retry_count: 0,
        });
        ob.remove("nonexistent");
        assert_eq!(ob.pending().len(), 1);
    }

    #[test]
    fn multiple_entries() {
        let mut ob = Outbox::new();
        for i in 0..5 {
            ob.enqueue(OutboxEntry {
                peer_id: format!("p{i}"),
                message_id: format!("m{i}"),
                frame_bytes: vec![],
                created_at_ms: i,
                retry_count: 0,
            });
        }
        assert_eq!(ob.pending().len(), 5);
    }

    #[test]
    fn default_is_empty() {
        let ob: Outbox = Default::default();
        assert!(ob.pending().is_empty());
    }
}
