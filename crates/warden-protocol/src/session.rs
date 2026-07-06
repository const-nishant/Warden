#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

#[derive(Debug)]
pub struct ProtocolSession {
    pub peer_id: String,
    pub state: SessionState,
}

impl ProtocolSession {
    pub fn new(peer_id: String) -> Self {
        Self {
            peer_id,
            state: SessionState::Disconnected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_disconnected() {
        let s = ProtocolSession::new("peer1".into());
        assert_eq!(s.peer_id, "peer1");
        assert_eq!(s.state, SessionState::Disconnected);
    }

    #[test]
    fn state_debug() {
        let s = ProtocolSession::new("p".into());
        let debug = format!("{s:?}");
        assert!(debug.contains("p"));
        assert!(debug.contains("Disconnected"));
    }

    #[test]
    fn session_state_clone_eq() {
        let states = vec![
            SessionState::Disconnected,
            SessionState::Connecting,
            SessionState::Connected,
            SessionState::Error("boom".into()),
        ];
        for s in &states {
            assert_eq!(s, s);
            assert_eq!(s.clone(), *s);
        }
    }

    #[test]
    fn session_state_debug() {
        let s = SessionState::Error("fail".into());
        let d = format!("{s:?}");
        assert!(d.contains("fail"));
    }
}
