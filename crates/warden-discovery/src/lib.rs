//! Kademlia DHT peer discovery and NAT traversal.
//!
//! Uses `libp2p` for Kademlia, Identify, Ping, DCUtR hole-punching,
//! Circuit Relay, AutoNAT, and UPnP. [`DiscoveryNode`] wraps the libp2p
//! swarm and exposes announce/resolve/bootstrap commands via a channel.

pub mod error;
pub mod node;

pub use error::DiscoveryError;
pub use node::DiscoveryNode;
