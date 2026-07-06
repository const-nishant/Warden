use std::fmt;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/warden.v1.rs"));
}

pub type TimestampMillis = i64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PeerId(String);

impl PeerId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct MessageId(pub uuid::Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_id_new_and_display() {
        let id = PeerId::new("abc123");
        assert_eq!(id.as_str(), "abc123");
        assert_eq!(id.to_string(), "abc123");
    }

    #[test]
    fn peer_id_clone_eq() {
        let a = PeerId::new("peer1");
        let b = PeerId::new("peer1");
        let c = PeerId::new("peer2");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn message_id_default_and_new() {
        let a = MessageId::new();
        let b = MessageId::default();
        assert_ne!(a.0, b.0);
    }

    #[test]
    fn message_id_display() {
        let id = MessageId::new();
        let s = id.to_string();
        assert_eq!(s.len(), 36);
        assert_eq!(s.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn timestamp_millis_type() {
        let ts: TimestampMillis = 1_700_000_000_000;
        assert!(ts > 0);
    }
}
