use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use russh::server::{Auth, Handler, Msg, Session};
use russh::{Channel, ChannelId};
use russh_cryptovec::CryptoVec;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};

use crate::error::TransportError;
use crate::session::{ChatSession, SessionEvent};

use bytes::Bytes;

struct SharedState {
    channels: HashMap<ChannelId, mpsc::Sender<SessionEvent>>,
    new_session_tx: mpsc::Sender<ChatSession>,
}

pub struct ServerHandler {
    state: Arc<Mutex<SharedState>>,
}

impl ServerHandler {
    pub fn new(new_session_tx: mpsc::Sender<ChatSession>) -> Self {
        Self {
            state: Arc::new(Mutex::new(SharedState {
                channels: HashMap::new(),
                new_session_tx,
            })),
        }
    }
}

#[async_trait]
impl Handler for ServerHandler {
    type Error = TransportError;

    async fn auth_publickey_offered(
        &mut self,
        _user: &str,
        _public_key: &russh_keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _public_key: &russh_keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn auth_succeeded(&mut self, _session: &mut Session) -> Result<(), Self::Error> {
        tracing::debug!("client authenticated");
        Ok(())
    }

    async fn channel_open_session(
        &mut self,
        _channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    async fn exec_request(
        &mut self,
        channel: ChannelId,
        _data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_failure(channel);
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_failure(channel);
        Ok(())
    }

    async fn env_request(
        &mut self,
        _channel: ChannelId,
        _variable_name: &str,
        _variable_value: &str,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn subsystem_request(
        &mut self,
        channel: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if name != "chat" {
            session.channel_failure(channel);
            return Ok(());
        }

        session.channel_success(channel);

        let (app_tx, mut app_rx) = mpsc::channel::<Bytes>(64);
        let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(64);

        let handle = session.handle();
        let state = self.state.clone();

        {
            let mut guard = state.lock().await;
            guard.channels.insert(channel, event_tx.clone());
        }

        let chat_session = ChatSession {
            peer_id: "connected".to_string(),
            sender: app_tx,
            receiver: event_rx,
        };

        {
            let guard = state.lock().await;
            let _ = guard.new_session_tx.send(chat_session).await;
        }

        let state_clone = state.clone();
        tokio::spawn(async move {
            while let Some(data) = app_rx.recv().await {
                let cv = CryptoVec::from_slice(&data);
                if handle.data(channel, cv).await.is_err() {
                    break;
                }
            }
            let mut guard = state_clone.lock().await;
            guard.channels.remove(&channel);
        });

        Ok(())
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let guard = self.state.lock().await;
        if let Some(tx) = guard.channels.get(&channel) {
            let _ = tx
                .send(SessionEvent::Message(Bytes::copy_from_slice(data)))
                .await;
        }
        Ok(())
    }

    async fn channel_close(
        &mut self,
        channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut guard = self.state.lock().await;
        if let Some(tx) = guard.channels.remove(&channel) {
            let _ = tx.send(SessionEvent::Disconnected).await;
        }
        Ok(())
    }

    async fn channel_eof(
        &mut self,
        channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let guard = self.state.lock().await;
        if let Some(tx) = guard.channels.get(&channel) {
            let _ = tx.send(SessionEvent::Disconnected).await;
        }
        Ok(())
    }
}

pub async fn start_server(
    bind_addr: &str,
    port: u16,
    new_session_tx: mpsc::Sender<ChatSession>,
) -> Result<(), TransportError> {
    let addr = format!("{bind_addr}:{port}");
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(TransportError::Io)?;

    tracing::info!("SSH server listening on {addr}");

    // Generate a server host key (Ed25519)
    let host_key = russh_keys::key::KeyPair::generate_ed25519();
    let mut config = russh::server::Config::default();
    config.keys.push(host_key);
    let config = Arc::new(config);

    loop {
        let (stream, peer_addr) = listener
            .accept()
            .await
            .map_err(TransportError::Io)?;

        let handler = ServerHandler::new(new_session_tx.clone());
        let peer = peer_addr;
        let config = config.clone();

        tokio::spawn(async move {
            tracing::debug!("accepted connection from {peer}");
            match russh::server::run_stream(config, stream, handler).await {
                Ok(session) => {
                    if let Err(e) = session.await {
                        tracing::debug!("session ended: {e}");
                    }
                }
                Err(e) => {
                    tracing::warn!("failed to establish session: {e}");
                }
            }
        });
    }
}
