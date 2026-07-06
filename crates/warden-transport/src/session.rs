use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum SessionEvent {
    Message(Bytes),
    Disconnected,
}

#[derive(Debug)]
pub struct ChatSession {
    pub peer_id: String,
    pub sender: mpsc::Sender<Bytes>,
    pub receiver: mpsc::Receiver<SessionEvent>,
}

impl ChatSession {
    pub fn new(
        peer_id: String,
        sender: mpsc::Sender<Bytes>,
        receiver: mpsc::Receiver<SessionEvent>,
    ) -> Self {
        Self {
            peer_id,
            sender,
            receiver,
        }
    }

    pub async fn send(&mut self, data: Bytes) -> Result<(), TransportError> {
        self.sender.send(data).await.map_err(|_| {
            TransportError::Connection("channel closed".into())
        })
    }

    pub async fn recv(&mut self) -> Option<SessionEvent> {
        self.receiver.recv().await
    }
}

use crate::TransportError;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn session_send_recv() {
        let (tx, _app_rx) = mpsc::channel::<Bytes>(8);
        let (_event_tx, rx) = mpsc::channel::<SessionEvent>(8);
        let session = ChatSession::new("test_peer".into(), tx, rx);
        assert_eq!(session.peer_id, "test_peer");
    }

    #[tokio::test]
    async fn session_send_and_recv_message() {
        let (app_tx, _app_rx) = mpsc::channel::<Bytes>(8);
        let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(8);

        let mut session = ChatSession::new("p1".into(), app_tx, event_rx);

        let data = Bytes::from("hello");
        event_tx.send(SessionEvent::Message(data.clone())).await.unwrap();

        let received = session.recv().await;
        match received {
            Some(SessionEvent::Message(b)) => assert_eq!(b, "hello"),
            _ => panic!("expected message"),
        }

        let (app_tx2, mut app_rx2) = mpsc::channel::<Bytes>(8);
        let (_event_tx2, event_rx2) = mpsc::channel::<SessionEvent>(8);
        let mut session2 = ChatSession::new("p1".into(), app_tx2, event_rx2);
        session2.send(Bytes::from("reply")).await.unwrap();
        let sent = app_rx2.recv().await.unwrap();
        assert_eq!(sent, "reply");
    }

    #[tokio::test]
    async fn session_disconnect_event() {
        let (app_tx, _) = mpsc::channel::<Bytes>(8);
        let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(8);

        let mut session = ChatSession::new("p1".into(), app_tx, event_rx);

        event_tx.send(SessionEvent::Disconnected).await.unwrap();
        let received = session.recv().await;
        assert!(matches!(received, Some(SessionEvent::Disconnected)));
    }

    #[tokio::test]
    async fn session_recv_returns_none_on_close() {
        let (app_tx, _) = mpsc::channel::<Bytes>(8);
        let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(8);
        drop(event_tx);

        let mut session = ChatSession::new("p1".into(), app_tx, event_rx);
        let received = session.recv().await;
        assert!(received.is_none());
    }

    #[tokio::test]
    async fn session_send_fails_on_closed_channel() {
        let (app_tx, app_rx) = mpsc::channel::<Bytes>(8);
        drop(app_rx);

        let (_event_tx, event_rx) = mpsc::channel::<SessionEvent>(8);
        let mut session = ChatSession::new("p1".into(), app_tx, event_rx);

        let result = session.send(Bytes::from("data")).await;
        assert!(result.is_err());
    }
}
