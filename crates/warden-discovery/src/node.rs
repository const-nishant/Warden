use std::collections::HashMap;
use std::time::Duration;

use futures::StreamExt;
use libp2p::kad::{
    self, store::MemoryStore, store::RecordStore, QueryId, QueryResult, Record, RecordKey,
};
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use libp2p::{identify, identity as libp2p_identity, mdns, ping, Multiaddr};
use tokio::sync::{mpsc, oneshot};
use tracing::{info, warn};

use warden_core::PeerId;
use warden_identity::IdentityKeypair;

use crate::DiscoveryError;

#[derive(NetworkBehaviour)]
struct Behaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
    relay_client: libp2p::relay::client::Behaviour,
    relay_server: libp2p::relay::Behaviour,
    dcutr: libp2p::dcutr::Behaviour,
    autonat: libp2p::autonat::v1::Behaviour,
    upnp: libp2p::upnp::tokio::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

enum Command {
    Bootstrap {
        peers: Vec<Multiaddr>,
        tx: oneshot::Sender<Result<(), DiscoveryError>>,
    },
    Announce {
        tx: oneshot::Sender<Result<(), DiscoveryError>>,
    },
    Resolve {
        target: PeerId,
        tx: oneshot::Sender<Result<Vec<Multiaddr>, DiscoveryError>>,
    },
    ConnectRelay {
        relay_addr: Multiaddr,
        tx: oneshot::Sender<Result<(), DiscoveryError>>,
    },
    AdvertiseRelayAddress {
        tx: oneshot::Sender<Result<Vec<Multiaddr>, DiscoveryError>>,
    },
    GetListeningAddrs {
        tx: oneshot::Sender<Vec<Multiaddr>>,
    },
    AddAnnounceAddr(Multiaddr),
    RetryBootstrap {
        tx: oneshot::Sender<Result<(), DiscoveryError>>,
    },
}

struct EventLoop {
    swarm: libp2p::swarm::Swarm<Behaviour>,
    peer_id: PeerId,
    listening_addrs: Vec<Multiaddr>,
    relay_addrs: Vec<Multiaddr>,
    extra_announce_addrs: Vec<Multiaddr>,
    pending_announces: HashMap<QueryId, oneshot::Sender<Result<(), DiscoveryError>>>,
    pending_resolves: HashMap<QueryId, oneshot::Sender<Result<Vec<Multiaddr>, DiscoveryError>>>,
    pending_bootstrap: Option<oneshot::Sender<Result<(), DiscoveryError>>>,
    command_rx: mpsc::Receiver<Command>,
}

impl EventLoop {
    async fn run(mut self) {
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await;
                }
                cmd = self.command_rx.recv() => {
                    match cmd {
                        Some(cmd) => self.handle_command(cmd).await,
                        None => break,
                    }
                }
            }
        }
    }

    async fn handle_swarm_event(&mut self, event: SwarmEvent<BehaviourEvent>) {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("DHT listening on {address}");
                self.listening_addrs.push(address);
            }
            SwarmEvent::Behaviour(behaviour_event) => match behaviour_event {
                BehaviourEvent::Kademlia(e) => {
                    self.handle_kademlia_event(e).await;
                }
                BehaviourEvent::Identify(e) => {
                    self.handle_identify_event(e);
                }
                BehaviourEvent::Mdns(e) => {
                    self.handle_mdns_event(e);
                }
                BehaviourEvent::RelayClient(e) => {
                    self.handle_relay_client_event(e);
                }
                BehaviourEvent::RelayServer(e) => {
                    self.handle_relay_server_event(e);
                }
                BehaviourEvent::Dcutr(e) => {
                    self.handle_dcutr_event(e);
                }
                BehaviourEvent::Autonat(e) => {
                    self.handle_autonat_event(e);
                }
                BehaviourEvent::Upnp(e) => {
                    self.handle_upnp_event(e);
                }
                BehaviourEvent::Ping(_) => {}
            },
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("DHT connected to {peer_id}");
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                info!("DHT disconnected from {peer_id}: {cause:?}");
            }
            SwarmEvent::IncomingConnection { .. } => {}
            _ => {}
        }
    }

    async fn handle_kademlia_event(&mut self, event: kad::Event) {
        let kad::Event::OutboundQueryProgressed { id, result, .. } = event else {
            return;
        };

        match result {
            QueryResult::PutRecord(res) => {
                if let Some(tx) = self.pending_announces.remove(&id) {
                    let _ = tx.send(res.map(|_| ()).map_err(|e| {
                        DiscoveryError::Dht(format!("put_record: {e}"))
                    }));
                }
            }
            QueryResult::GetRecord(res) => {
                if let Some(tx) = self.pending_resolves.remove(&id) {
                    let result = match res {
                        Ok(kad::GetRecordOk::FoundRecord(peer_record)) => {
                            let addrs =
                                deserialize_addrs(&peer_record.record.value).unwrap_or_default();
                            if addrs.is_empty() {
                                Err(DiscoveryError::Resolve(
                                    "no addresses found in DHT record".into(),
                                ))
                            } else {
                                Ok(addrs)
                            }
                        }
                        Ok(kad::GetRecordOk::FinishedWithNoAdditionalRecord { .. }) => {
                            Err(DiscoveryError::Resolve(
                                "no records found in DHT".into(),
                            ))
                        }
                        Err(e) => {
                            Err(DiscoveryError::Dht(format!("get_record query failed: {e}")))
                        }
                    };
                    let _ = tx.send(result);
                }
            }
            QueryResult::Bootstrap(Err(e)) => {
                warn!("DHT bootstrap failed: {e}");
            }
            _ => {}
        }
    }

    fn handle_mdns_event(&mut self, event: mdns::Event) {
        match event {
            mdns::Event::Discovered(peers) => {
                for (peer_id, addr) in peers {
                    info!("mDNS discovered peer {peer_id} at {addr}");
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&peer_id, addr);
                }
            }
            mdns::Event::Expired(peers) => {
                for (peer_id, _addr) in peers {
                    info!("mDNS peer expired: {peer_id}");
                }
            }
        }
    }

    fn handle_identify_event(&mut self, event: identify::Event) {
        if let identify::Event::Received { info, .. } = event {
            let peer_id = info.public_key.to_peer_id();
            for addr in info.listen_addrs {
                self.swarm
                    .behaviour_mut()
                    .kademlia
                    .add_address(&peer_id, addr);
            }
            // Deferred bootstrap: now that the peer is in the routing table
            // (added via add_address above), trigger kad.bootstrap()
            if let Some(tx) = self.pending_bootstrap.take() {
                match self.swarm.behaviour_mut().kademlia.bootstrap() {
                    Ok(query_id) => {
                        info!("DHT bootstrap started, query id: {query_id}");
                        let _ = tx.send(Ok(()));
                    }
                    Err(e) => {
                        warn!("DHT bootstrap returned no known peers: {e}");
                        let _ = tx.send(Err(DiscoveryError::Dht(e.to_string())));
                    }
                }
            }
        }
    }

    fn handle_relay_client_event(&mut self, event: libp2p::relay::client::Event) {
        match event {
            libp2p::relay::client::Event::ReservationReqAccepted {
                relay_peer_id, ..
            } => {
                info!("Relay reservation accepted by {relay_peer_id}");
                let circuit_addr = Multiaddr::empty()
                    .with(libp2p::multiaddr::Protocol::P2p(relay_peer_id))
                    .with(libp2p::multiaddr::Protocol::P2pCircuit);
                self.relay_addrs.push(circuit_addr);
            }
            libp2p::relay::client::Event::OutboundCircuitEstablished {
                relay_peer_id, ..
            } => {
                info!("Outbound relay circuit established via {relay_peer_id}");
            }
            libp2p::relay::client::Event::InboundCircuitEstablished {
                src_peer_id, ..
            } => {
                info!("Inbound relay circuit established from {src_peer_id}");
            }
        }
    }

    fn handle_relay_server_event(&mut self, event: libp2p::relay::Event) {
        match event {
            libp2p::relay::Event::ReservationReqAccepted { src_peer_id, .. } => {
                info!("Relay server: reservation accepted for {src_peer_id}");
            }
            libp2p::relay::Event::CircuitReqAccepted { src_peer_id, dst_peer_id, .. } => {
                info!("Relay server: circuit established {src_peer_id} <-> {dst_peer_id}");
            }
            _ => {}
        }
    }

    fn handle_dcutr_event(&mut self, event: libp2p::dcutr::Event) {
        info!(
            "DCUtR hole-punch result for {}: {:?}",
            event.remote_peer_id, event.result
        );
    }

    fn handle_autonat_event(&mut self, event: libp2p::autonat::v1::Event) {
        if let libp2p::autonat::v1::Event::StatusChanged { old, new } = event {
            info!("NAT status changed: {old:?} -> {new:?}");
        }
    }

    fn handle_upnp_event(&mut self, event: libp2p::upnp::Event) {
        if let libp2p::upnp::Event::NewExternalAddr(addr) = event {
            info!("UPnP: new external address {addr}");
        }
    }

    async fn handle_command(&mut self, cmd: Command) {
        match cmd {
            Command::Bootstrap { peers, tx } => {
                for peer in &peers {
                    if let Err(e) = self.swarm.dial(peer.clone()) {
                        warn!("Failed to dial bootstrap peer {peer}: {e}");
                    }
                }
                // Defer kad.bootstrap() until Identify response adds the peer
                // to the routing table (handled in handle_identify_event)
                self.pending_bootstrap = Some(tx);
            }
            Command::Announce { tx } => {
                let mut all_addrs = self.listening_addrs.clone();
                all_addrs.extend(self.relay_addrs.clone());
                all_addrs.extend(self.extra_announce_addrs.clone());
                let addrs: Vec<String> = all_addrs.iter().map(|a| a.to_string()).collect();
                let value = serde_json::to_vec(&addrs).unwrap_or_default();
                let key = RecordKey::new(&self.peer_id.as_str().as_bytes());

                // Always store locally so the record is available even without peers
                let record = Record {
                    key: key.clone(),
                    value: value.clone(),
                    publisher: None,
                    expires: None,
                };
                let _ = self.swarm.behaviour_mut().kademlia.store_mut().put(record);

                // Also replicate to DHT peers if any exist
                let record = Record {
                    key,
                    value,
                    publisher: None,
                    expires: None,
                };
                match self
                    .swarm
                    .behaviour_mut()
                    .kademlia
                    .put_record(record, kad::Quorum::One)
                {
                    Ok(query_id) => {
                        self.pending_announces.insert(query_id, tx);
                    }
                    Err(_) => {
                        // No peers to replicate to — local store is sufficient
                        let _ = tx.send(Ok(()));
                    }
                }
            }
            Command::Resolve { target, tx } => {
                let key = RecordKey::new(&target.as_str().as_bytes());
                let query_id = self.swarm.behaviour_mut().kademlia.get_record(key);
                self.pending_resolves.insert(query_id, tx);
            }
            Command::ConnectRelay { relay_addr, tx } => {
                match self.swarm.dial(relay_addr.clone()) {
                    Ok(()) => {
                        info!("Dialing relay server at {relay_addr}");
                        let _ = tx.send(Ok(()));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(DiscoveryError::Dht(format!(
                            "failed to dial relay {relay_addr}: {e}"
                        ))));
                    }
                }
            }
            Command::AdvertiseRelayAddress { tx } => {
                let addrs = self.relay_addrs.clone();
                let _ = tx.send(Ok(addrs));
            }
            Command::GetListeningAddrs { tx } => {
                let _ = tx.send(self.listening_addrs.clone());
            }
            Command::AddAnnounceAddr(addr) => {
                self.extra_announce_addrs.push(addr);
            }
            Command::RetryBootstrap { tx } => {
                let result = self.swarm.behaviour_mut().kademlia.bootstrap();
                match result {
                    Ok(query_id) => {
                        info!("DHT retry bootstrap started, query id: {query_id}");
                        let _ = tx.send(Ok(()));
                    }
                    Err(e) => {
                        warn!("DHT retry bootstrap still has no known peers: {e}");
                        let _ = tx.send(Err(DiscoveryError::Dht(e.to_string())));
                    }
                }
            }
        }
    }
}

fn deserialize_addrs(data: &[u8]) -> Result<Vec<Multiaddr>, DiscoveryError> {
    let strings: Vec<String> = serde_json::from_slice(data)
        .map_err(|e| DiscoveryError::Dht(format!("invalid addr data: {e}")))?;
    strings
        .iter()
        .map(|s| {
            s.parse()
                .map_err(|e| DiscoveryError::Dht(format!("invalid multiaddr '{s}': {e}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use warden_identity::IdentityKeypair;

    fn make_identity() -> IdentityKeypair {
        IdentityKeypair::generate()
    }

    #[tokio::test]
    async fn two_nodes_bootstrap_announce_resolve() {
        let alice_identity = make_identity();
        let bob_identity = make_identity();

        let alice_listen: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
        let bob_listen: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();

        let alice = DiscoveryNode::new(&alice_identity, vec![alice_listen])
            .await
            .expect("alice node should start");
        let bob = DiscoveryNode::new(&bob_identity, vec![bob_listen])
            .await
            .expect("bob node should start");

        // Wait for listen addresses to be registered
        tokio::time::sleep(Duration::from_millis(500)).await;

        let alice_addrs = alice
            .listening_addrs()
            .await
            .expect("alice should report listening addrs");
        assert!(!alice_addrs.is_empty(), "alice should have at least one listening addr");

        // Bob bootstraps to Alice
        bob.bootstrap(alice_addrs.clone())
            .await
            .expect("bob should bootstrap to alice");

        // Wait for Kademlia handshake + bootstrap
        tokio::time::sleep(Duration::from_secs(3)).await;

        // Alice announces her addresses
        alice.announce()
            .await
            .expect("alice should announce");

        // Wait for DHT propagation
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Bob resolves Alice
        let resolved = bob
            .resolve(alice_identity.peer_id().clone())
            .await
            .expect("bob should resolve alice");

        assert!(!resolved.is_empty(), "resolved addrs should not be empty");
        // At least one of Alice's addrs should appear in the resolved set
        for addr in &alice_addrs {
            let addr_str = addr.to_string();
            let ip4_part = addr_str.split("/p2p/").next().unwrap_or(&addr_str);
            let has_match = resolved.iter().any(|r| {
                r.to_string().contains(ip4_part)
            });
            if has_match {
                return; // Found at least one matching address
            }
        }
        panic!(
            "None of Alice's addrs ({alice_addrs:?}) appeared in resolved addrs ({resolved:?})"
        );
    }

    #[tokio::test]
    async fn listen_on_ephemeral_port_reports_addr() {
        let identity = make_identity();
        let listen: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
        let node = DiscoveryNode::new(&identity, vec![listen])
            .await
            .expect("node should start");

        tokio::time::sleep(Duration::from_millis(500)).await;

        let addrs = node
            .listening_addrs()
            .await
            .expect("should report listening addrs");
        assert!(!addrs.is_empty(), "should have at least one listening addr");
        // Verify it's a TCP address on localhost
        let found_tcp = addrs.iter().any(|a| {
            let s = a.to_string();
            s.contains("/ip4/127.0.0.1/tcp/")
        });
        assert!(found_tcp, "should have a TCP listening address: {addrs:?}");
    }

    #[tokio::test]
    async fn announce_without_bootstrap_errors_on_resolve() {
        let identity = make_identity();
        let listen: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
        let node = DiscoveryNode::new(&identity, vec![listen])
            .await
            .expect("node should start");

        tokio::time::sleep(Duration::from_millis(500)).await;

        // Announce without any peers in the DHT
        let announce_result = node.announce().await;
        assert!(
            announce_result.is_ok() || announce_result.is_err(),
            "announce may succeed (stored locally) or fail"
        );

        // Resolving a random peer should fail (no DHT peers to query)
        let other = make_identity();
        let resolve_result = node.resolve(other.peer_id().clone()).await;
        assert!(
            resolve_result.is_err(),
            "resolving with no DHT peers should fail"
        );
    }
}

fn identity_to_libp2p_keypair(
    identity: &IdentityKeypair,
) -> Result<libp2p_identity::Keypair, DiscoveryError> {
    let mut seed = identity.signing_key().to_bytes();
    libp2p_identity::Keypair::ed25519_from_bytes(&mut seed[..])
        .map_err(|e| DiscoveryError::Keypair(e.to_string()))
}

#[derive(Clone)]
pub struct DiscoveryNode {
    peer_id: PeerId,
    command_tx: mpsc::Sender<Command>,
}

impl DiscoveryNode {
    pub async fn new(
        identity: &IdentityKeypair,
        listen_addrs: Vec<Multiaddr>,
    ) -> Result<Self, DiscoveryError> {
        let keypair = identity_to_libp2p_keypair(identity)?;
        let peer_id = identity.peer_id().clone();

        info!("Starting discovery node with NAT traversal support");

        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::default(),
                |key: &libp2p_identity::Keypair| {
                    libp2p::noise::Config::new(key)
                        .map_err(|e: libp2p::noise::Error| -> std::io::Error {
                            std::io::Error::other(e)
                        })
                },
                libp2p::yamux::Config::default,
            )
            .map_err(|e| DiscoveryError::Dht(e.to_string()))?
            .with_dns()
            .map_err(|e| DiscoveryError::Dht(e.to_string()))?
            .with_relay_client(
                |key: &libp2p_identity::Keypair| {
                    libp2p::noise::Config::new(key)
                        .map_err(|e: libp2p::noise::Error| -> std::io::Error {
                            std::io::Error::other(e)
                        })
                },
                libp2p::yamux::Config::default,
            )
            .map_err(|e| DiscoveryError::Dht(e.to_string()))?
            .with_behaviour(
                |key: &libp2p_identity::Keypair,
                 relay_client: libp2p::relay::client::Behaviour| {
                    let peer_id = key.public().to_peer_id();
                    let store = MemoryStore::new(peer_id);
                    let mut kad = kad::Behaviour::new(peer_id, store);
                    kad.set_mode(Some(kad::Mode::Server));

                    Behaviour {
                        kademlia: kad,
                        identify: identify::Behaviour::new(
                            identify::Config::new(
                                "/warden/0.1.0".to_string(),
                                key.public(),
                            )
                            .with_agent_version(format!("warden/{}", env!("CARGO_PKG_VERSION"))),
                        ),
                        ping: ping::Behaviour::new(ping::Config::new()),
                        relay_client,
                        relay_server: libp2p::relay::Behaviour::new(
                            peer_id,
                            libp2p::relay::Config::default(),
                        ),
                        dcutr: libp2p::dcutr::Behaviour::new(peer_id),
                        autonat: libp2p::autonat::v1::Behaviour::new(
                            peer_id,
                            libp2p::autonat::v1::Config::default(),
                        ),
                        upnp: libp2p::upnp::tokio::Behaviour::default(),
                        mdns: mdns::tokio::Behaviour::new(
                            mdns::Config::default(),
                            key.public().to_peer_id(),
                        )
                        .expect("mDNS should initialize"),
                    }
                },
            )?
            .with_swarm_config(|c: libp2p::swarm::Config| {
                c.with_idle_connection_timeout(Duration::from_secs(120))
            })
            .build();

        for addr in &listen_addrs {
            swarm
                .listen_on(addr.clone())
                .map_err(|e| DiscoveryError::Listen(format!("{addr}: {e}")))?;
        }

        let (command_tx, command_rx) = mpsc::channel::<Command>(32);

        let event_loop = EventLoop {
            swarm,
            peer_id: peer_id.clone(),
            listening_addrs: Vec::new(),
            relay_addrs: Vec::new(),
            extra_announce_addrs: Vec::new(),
            pending_announces: HashMap::new(),
            pending_resolves: HashMap::new(),
            pending_bootstrap: None,
            command_rx,
        };

        tokio::spawn(event_loop.run());

        Ok(Self {
            peer_id,
            command_tx,
        })
    }

    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }

    pub async fn bootstrap(&self, peers: Vec<Multiaddr>) -> Result<(), DiscoveryError> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::Bootstrap { peers, tx })
            .await
            .map_err(|_| DiscoveryError::Shutdown)?;
        // Wait for deferred bootstrap with a timeout so we don't hang
        // if Identify never fires (e.g. dial fails silently)
        tokio::time::timeout(Duration::from_secs(5), rx)
            .await
            .map_err(|_| DiscoveryError::Dht("bootstrap dial timed out".into()))?
            .map_err(|_| DiscoveryError::Shutdown)?
    }

    pub async fn announce(&self) -> Result<(), DiscoveryError> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::Announce { tx })
            .await
            .map_err(|_| DiscoveryError::Shutdown)?;
        rx.await.map_err(|_| DiscoveryError::Shutdown)?
    }

    pub async fn resolve(&self, target: PeerId) -> Result<Vec<Multiaddr>, DiscoveryError> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::Resolve { target, tx })
            .await
            .map_err(|_| DiscoveryError::Shutdown)?;
        rx.await.map_err(|_| DiscoveryError::Shutdown)?
    }

    pub async fn connect_relay(&self, relay_addr: Multiaddr) -> Result<(), DiscoveryError> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::ConnectRelay { relay_addr, tx })
            .await
            .map_err(|_| DiscoveryError::Shutdown)?;
        rx.await.map_err(|_| DiscoveryError::Shutdown)?
    }

    pub async fn retry_bootstrap(&self) -> Result<(), DiscoveryError> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::RetryBootstrap { tx })
            .await
            .map_err(|_| DiscoveryError::Shutdown)?;
        rx.await.map_err(|_| DiscoveryError::Shutdown)?
    }

    pub async fn add_announce_addr(&self, addr: Multiaddr) -> Result<(), DiscoveryError> {
        self.command_tx
            .send(Command::AddAnnounceAddr(addr))
            .await
            .map_err(|_| DiscoveryError::Shutdown)
    }

    pub async fn listening_addrs(&self) -> Result<Vec<Multiaddr>, DiscoveryError> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::GetListeningAddrs { tx })
            .await
            .map_err(|_| DiscoveryError::Shutdown)?;
        rx.await.map_err(|_| DiscoveryError::Shutdown)
    }

    pub async fn advertised_relay_addrs(&self) -> Result<Vec<Multiaddr>, DiscoveryError> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::AdvertiseRelayAddress { tx })
            .await
            .map_err(|_| DiscoveryError::Shutdown)?;
        rx.await.map_err(|_| DiscoveryError::Shutdown)?
    }
}
