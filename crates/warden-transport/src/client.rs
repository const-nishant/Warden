use std::sync::Arc;

use async_trait::async_trait;
use russh::client::{self, Handler};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use crate::error::TransportError;
use crate::known_hosts::KnownHosts;
use crate::session::{ChatSession, SessionEvent};

use bytes::Bytes;

pub struct ClientHandler {
    host: String,
    known_hosts: Option<KnownHosts>,
}

impl ClientHandler {
    pub fn new(host: String, known_hosts: Option<KnownHosts>) -> Self {
        Self { host, known_hosts }
    }
}

#[async_trait]
impl Handler for ClientHandler {
    type Error = TransportError;

    async fn check_server_key(
        &mut self,
        server_public_key: &russh_keys::key::PublicKey,
    ) -> Result<bool, Self::Error> {
        match &mut self.known_hosts {
            Some(kh) => kh
                .verify(&self.host, server_public_key)
                .map_err(TransportError::Auth),
            None => Ok(true),
        }
    }
}

pub async fn connect(
    addr: &str,
) -> Result<ChatSession, TransportError> {
    connect_optional(addr, None).await
}

pub async fn connect_with_known_hosts(
    addr: &str,
    known_hosts: KnownHosts,
) -> Result<ChatSession, TransportError> {
    connect_optional(addr, Some(known_hosts)).await
}

async fn connect_optional(
    addr: &str,
    known_hosts: Option<KnownHosts>,
) -> Result<ChatSession, TransportError> {
    let addr: std::net::SocketAddr = addr
        .parse()
        .map_err(|e| TransportError::Connection(format!("invalid address: {e}")))?;

    let config = Arc::new(client::Config::default());
    let handler = ClientHandler::new(addr.to_string(), known_hosts);

    let mut handle = client::connect(config, addr, handler)
        .await
        .map_err(|e| TransportError::Connection(e.to_string()))?;

    let key_pair = Arc::new(
        russh_keys::key::KeyPair::generate_ed25519(),
    );
    let auth_ok = handle
        .authenticate_publickey("warden", key_pair)
        .await
        .map_err(|e| TransportError::Auth(e.to_string()))?;

    if !auth_ok {
        return Err(TransportError::Auth("public key authentication failed".into()));
    }

    let channel = handle
        .channel_open_session()
        .await
        .map_err(|e| TransportError::Connection(e.to_string()))?;

    channel
        .request_subsystem(true, "chat")
        .await
        .map_err(|e| TransportError::Subsystem(e.to_string()))?;

    let stream = channel.into_stream();
    let (mut reader, mut writer) = tokio::io::split(stream);

    let (app_tx, mut app_rx) = mpsc::channel::<Bytes>(64);
    let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(64);

    let session = ChatSession {
        peer_id: addr.to_string(),
        sender: app_tx,
        receiver: event_rx,
    };

    let et = event_tx.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 65536];
        loop {
            match reader.read(&mut buf).await {
                Ok(0) | Err(_) => {
                    let _ = et.send(SessionEvent::Disconnected).await;
                    break;
                }
                Ok(n) => {
                    if et
                        .send(SessionEvent::Message(Bytes::copy_from_slice(&buf[..n])))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(data) = app_rx.recv().await {
            if writer.write_all(&data).await.is_err() {
                break;
            }
        }
    });

    Ok(session)
}
