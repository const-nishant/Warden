//! SSH transport layer for Warden.
//!
//! Each peer runs an embedded SSH server via `russh` that exposes the `chat`
//! subsystem only (no shell, exec, or port forwarding). The [`connect`] function
//! connects to a remote peer and opens the `chat` subsystem.
//!
//! Also provides [`KnownHosts`] for TOFU server-key verification.

pub mod client;
pub mod config;
pub mod error;
pub mod known_hosts;
pub mod server;
pub mod session;

pub use client::{connect, connect_with_known_hosts};
pub use config::ServerConfig;
pub use error::TransportError;
pub use known_hosts::KnownHosts;
pub use server::start_server;
pub use session::{ChatSession, SessionEvent};
