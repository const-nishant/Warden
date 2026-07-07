use std::time::{Duration, SystemTime};

use clap::{Parser, Subcommand};
use tokio::sync::mpsc;
use tokio::io::{AsyncBufReadExt, BufReader};
use warden_transport::SessionEvent;
use bytes::Bytes;

#[derive(Parser)]
#[command(name = "warden", about = "Peer-to-peer decentralized chat over SSH")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a new identity keypair
    Identity {
        #[command(subcommand)]
        action: IdentityAction,
    },
    /// Listen for incoming SSH connections (chat server)
    Listen {
        /// Port to bind the SSH server
        #[arg(long, default_value = "2222")]
        port: u16,
    },
    /// Connect to a peer via SSH (accepts IP:port or PeerID with DHT resolution)
    Connect {
        /// Peer address (IP:port) or PeerID (base58, resolved via DHT)
        addr: String,
        /// Bootstrap peer multiaddrs for DHT resolution (can be repeated)
        #[arg(long = "bootstrap", short)]
        bootstrap: Vec<String>,
        /// Relay server multiaddr to use for DHT connectivity (can be repeated)
        #[arg(long = "relay", short = 'r')]
        relay: Vec<String>,
    },
    /// DHT peer discovery commands
    Discovery {
        #[command(subcommand)]
        action: DiscoveryAction,
    },
    /// Manage contacts
    Contacts {
        #[command(subcommand)]
        action: ContactsAction,
    },
    /// Show message history with a peer
    History {
        /// Peer ID to show history with
        peer_id: String,
        #[arg(long, default_value = "50")]
        limit: usize,
    },
    /// Launch the terminal UI (TUI) with built-in DHT node
    Tui {
        /// SSH chat server port
        #[arg(long, default_value = "2222")]
        port: u16,
        /// DHT listener port (0 = OS-assigned)
        #[arg(long = "dht-port", default_value = "0", hide = true)]
        dht_port: u16,
        /// Bootstrap peer multiaddr (advanced)
        #[arg(long = "bootstrap", short, hide = true)]
        bootstrap: Vec<String>,
        /// Relay server multiaddr (advanced)
        #[arg(long = "relay", short = 'r', hide = true)]
        relay: Vec<String>,
    },
    /// Outbox management (store-and-forward)
    Outbox {
        #[command(subcommand)]
        action: OutboxAction,
    },
    /// Group chat management
    Groups {
        #[command(subcommand)]
        action: GroupsAction,
    },
    /// Run the full daemon (SSH server + DHT node + relay)
    Daemon {
        /// Port for SSH chat server
        #[arg(long, default_value = "2222")]
        ssh_port: u16,
        /// Port for DHT listener
        #[arg(long, default_value = "3333")]
        dht_port: u16,
        /// Bootstrap peer multiaddr (can be repeated)
        #[arg(long = "bootstrap", short)]
        bootstrap: Vec<String>,
        /// Relay server multiaddr to connect through (can be repeated)
        #[arg(long = "relay", short = 'r')]
        relay: Vec<String>,
    },
}

#[derive(Subcommand)]
enum IdentityAction {
    /// Generate and save a new keypair
    Init,
    /// Display the current identity
    Show,
}

#[derive(Subcommand)]
enum DiscoveryAction {
    /// Announce our PeerID on the DHT and keep the node running
    Announce {
        #[arg(long, default_value = "3333")]
        port: u16,
        #[arg(long = "bootstrap", short)]
        bootstrap: Vec<String>,
        #[arg(long = "relay", short = 'r')]
        relay: Vec<String>,
    },
    /// Resolve a PeerID to its advertised multiaddrs
    Resolve {
        peer_id: String,
        #[arg(long = "bootstrap", short)]
        bootstrap: Vec<String>,
    },
    /// Connect to a relay server
    RelayConnect {
        relay_addr: String,
        #[arg(long = "bootstrap", short)]
        bootstrap: Vec<String>,
    },
    /// Print our relay addresses
    RelayAddrs,
}

#[derive(Subcommand)]
enum OutboxAction {
    /// Send a message to a peer (queued for delivery)
    Send {
        peer_id: String,
        message: String,
    },
    /// Retry sending all pending outbox messages
    Flush,
    /// List pending outbox entries
    List,
}

#[derive(Subcommand)]
enum GroupsAction {
    /// Create a new group
    Create {
        name: String,
        #[arg(required = true)]
        members: Vec<String>,
    },
    /// List all groups
    List,
    /// Show group members
    Members {
        group_id: String,
    },
    /// Send a message to all group members (fanout)
    Send {
        group_id: String,
        message: String,
    },
}

#[derive(Subcommand)]
enum ContactsAction {
    /// Add a contact
    Add {
        peer_id: String,
        #[arg(long)]
        alias: Option<String>,
    },
    /// List all contacts
    List,
    /// Remove a contact
    Remove {
        peer_id: String,
    },
}

fn warden_dir() -> anyhow::Result<std::path::PathBuf> {
    let dir = home::home_dir()
        .ok_or_else(|| anyhow::anyhow!("cannot find home directory"))?
        .join(".warden");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn identity_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(warden_dir()?.join("identity"))
}

fn known_hosts_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(warden_dir()?.join("known_hosts.json"))
}

fn known_peers_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(warden_dir()?.join("known_peers.json"))
}

fn load_known_peers() -> Vec<String> {
    let path = match known_peers_path() {
        Ok(p) => p,
        _ => return vec![],
    };
    match std::fs::read_to_string(&path) {
        Ok(s) => s.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect(),
        _ => vec![],
    }
}

fn save_known_peers(addrs: &[String]) {
    let path = match known_peers_path() {
        Ok(p) => p,
        _ => return,
    };
    let mut existing: Vec<String> = load_known_peers();
    for addr in addrs {
        let a = addr.trim().to_string();
        if !a.is_empty() && !existing.contains(&a) {
            existing.push(a);
        }
    }
    if let Ok(mut f) = std::fs::File::create(&path) {
        use std::io::Write;
        let _ = existing.iter().try_for_each(|a| writeln!(f, "{a}"));
    }
}

async fn connect_with_default_known_hosts(addr: &str) -> anyhow::Result<warden_transport::ChatSession> {
    let known_hosts = warden_transport::KnownHosts::new(known_hosts_path()?);
    Ok(warden_transport::connect_with_known_hosts(addr, known_hosts).await?)
}

fn multiaddr_to_socketaddr(ma: &libp2p::Multiaddr) -> Option<std::net::SocketAddr> {
    use libp2p::multiaddr::Protocol;
    let mut ip = None;
    let mut port = None;
    for proto in ma.iter() {
        match proto {
            Protocol::Ip4(a) => ip = Some(std::net::IpAddr::V4(a)),
            Protocol::Ip6(a) => ip = Some(std::net::IpAddr::V6(a)),
            Protocol::Tcp(p) => port = Some(p),
            _ => {}
        }
    }
    Some(std::net::SocketAddr::new(ip?, port?))
}

async fn resolve_peer_id(
    peer_id: &str,
    bootstrap: &[String],
    relay: &[String],
) -> anyhow::Result<std::net::SocketAddr> {
    // Use an ephemeral keypair so the temporary DHT node does not conflict
    // with a running daemon that uses the same identity (self-dial would fail).
    let keypair = warden_identity::IdentityKeypair::generate();
    let bootstrap_addrs = parse_multiaddrs(bootstrap)?;
    let relay_addrs = parse_multiaddrs(relay)?;

    let node = warden_discovery::DiscoveryNode::new(
        &keypair,
        vec!["/ip4/127.0.0.1/tcp/0".parse().unwrap()],
    )
    .await?;

    for relay_addr in &relay_addrs {
        println!("Connecting to relay {relay_addr}...");
        let _ = node.connect_relay(relay_addr.clone()).await;
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    if !bootstrap_addrs.is_empty() {
        println!("Bootstrapping DHT to {} peers...", bootstrap_addrs.len());
        match node.bootstrap(bootstrap_addrs).await {
            Ok(()) => println!("DHT bootstrap ready."),
            Err(e) => println!("DHT bootstrap dial: {e} (will retry)"),
        }
        tokio::time::sleep(std::time::Duration::from_secs(4)).await;
        match node.retry_bootstrap().await {
            Ok(()) => println!("DHT bootstrap ready."),
            Err(e) => println!("DHT bootstrap warning: {e} (proceeding anyway)"),
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    let target = warden_core::PeerId::new(peer_id);
    println!("Resolving {target} on DHT...");
    let addrs = node.resolve(target).await?;

    println!("Found {} address(es):", addrs.len());
    for addr in &addrs {
        println!("  {addr}");
    }

    // Persist resolved addresses as known peers for future auto-bootstrap
    let known: Vec<String> = addrs.iter().map(|a| a.to_string()).collect();
    save_known_peers(&known);

    let mut fallback = None;
    for addr in &addrs {
        if let Some(sa) = multiaddr_to_socketaddr(addr) {
            if sa.port() == 2222 {
                println!("Resolved to {sa}");
                return Ok(sa);
            }
            if fallback.is_none() {
                fallback = Some(sa);
            }
        }
    }

    if let Some(sa) = fallback {
        let ssh = std::net::SocketAddr::new(sa.ip(), 2222);
        println!("Resolved to {} (via SSH port 2222)", sa.ip());
        return Ok(ssh);
    }

    Err(anyhow::anyhow!(
        "peer {peer_id} found on DHT but no valid IP:port address in {:?}",
        addrs
    ))
}

fn db_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(warden_dir()?.join("warden.db"))
}

fn open_db() -> anyhow::Result<warden_storage::Database> {
    let db = warden_storage::Database::open(
        &db_path()?.to_string_lossy(),
    )?;
    db.migrate()?;
    Ok(db)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Don't init tracing for TUI (it would corrupt the alternate screen)
    let is_tui = matches!(cli.command, Command::Tui { .. });
    if !is_tui {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .init();
    }

    match cli.command {
        Command::Identity { action } => match action {
            IdentityAction::Init => {
                let keypair = warden_identity::IdentityKeypair::generate();
                keypair.save(identity_path()?)?;
                println!("Identity saved to {}", identity_path()?.display());
                println!("PeerID: {}", keypair.peer_id());
            }
            IdentityAction::Show => {
                let keypair = warden_identity::IdentityKeypair::load(identity_path()?)?;
                println!("PeerID: {}", keypair.peer_id());
            }
        },
        Command::Listen { port } => {
            println!("Starting SSH server on port {port}...");
            let (tx, mut rx) = mpsc::channel::<warden_transport::ChatSession>(64);

            let server_task = tokio::spawn(async move {
                if let Err(e) = warden_transport::start_server("0.0.0.0", port, tx).await {
                    tracing::error!("Server error: {e}");
                }
            });

            println!("Waiting for connections...");
            while let Some(mut session) = rx.recv().await {
                let peer = session.peer_id.clone();
                if let Ok(db) = open_db() {
                    let _ = warden_storage::ContactStore::update_last_seen(
                        &db,
                        &peer,
                        chrono::Utc::now().timestamp_millis(),
                    );
                }
                println!("New session from {peer}");
                tokio::spawn(async move {
                    flush_outbox_for_peer(&mut session, &peer).await;
                    let db_task = open_db();
                    let conversation_id = conversation_id(&peer);
                    loop {
                        tokio::select! {
                            event = session.recv() => {
                                match event {
                                    Some(SessionEvent::Message(data)) => {
                                        let text = String::from_utf8_lossy(&data);
                                        println!("[{peer}] {text}");
                                        if let Ok(ref db) = db_task {
                                            let msg = warden_storage::StoredMessage {
                                                id: uuid::Uuid::new_v4().to_string(),
                                                conversation_id: conversation_id.clone(),
                                                sender_peer_id: peer.clone(),
                                                ciphertext: data.to_vec(),
                                                signature: None,
                                                frame_type: 1,
                                                timestamp_unix_ms: chrono::Utc::now().timestamp_millis(),
                                                delivered: true,
                                            };
                                            let _ = warden_storage::MessageStore::store_message(db, msg);
                                        }
                                    }
                                    Some(SessionEvent::Disconnected) => {
                                        println!("[{peer}] disconnected");
                                        break;
                                    }
                                    None => break,
                                }
                            }
                            line = read_stdin_line() => {
                                match line {
                                    Ok(line) => {
                                        if line.trim().eq_ignore_ascii_case("/quit") {
                                            break;
                                        }
                                        if let Ok(ref db) = db_task {
                                            let msg = warden_storage::StoredMessage {
                                                id: uuid::Uuid::new_v4().to_string(),
                                                conversation_id: conversation_id.clone(),
                                                sender_peer_id: String::new(),
                                                ciphertext: line.as_bytes().to_vec(),
                                                signature: None,
                                                frame_type: 1,
                                                timestamp_unix_ms: chrono::Utc::now().timestamp_millis(),
                                                delivered: true,
                                            };
                                            let _ = warden_storage::MessageStore::store_message(db, msg);
                                        }
                                        let _ = session.send(Bytes::from(line)).await;
                                    }
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                });
            }

            let _ = server_task.await;
        }
        Command::Connect { addr, bootstrap, relay } => {
            let addr = match resolve_peer_id(&addr, &bootstrap, &relay).await {
                Ok(sa) => sa.to_string(),
                Err(e) => {
                    if addr.parse::<std::net::SocketAddr>().is_ok() {
                        tracing::debug!("DHT resolution failed ({e}), using raw address");
                        addr
                    } else {
                        return Err(e.into());
                    }
                }
            };
            println!("Connecting to {addr}...");
            let mut session = connect_with_default_known_hosts(&addr).await?;
            let peer = session.peer_id.clone();
            let db = open_db();
            if let Ok(ref db) = db {
                let _ = warden_storage::ContactStore::update_last_seen(
                    db,
                    &peer,
                    chrono::Utc::now().timestamp_millis(),
                );
            }
            println!("Connected! Type /quit to exit.");
            flush_outbox_for_peer(&mut session, &peer).await;
            let conversation_id = conversation_id(&peer);

            loop {
                tokio::select! {
                    event = session.recv() => {
                        match event {
                            Some(SessionEvent::Message(data)) => {
                                let text = String::from_utf8_lossy(&data);
                                println!("[{peer}] {text}");
                                if let Ok(ref db) = db {
                                    let msg = warden_storage::StoredMessage {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        conversation_id: conversation_id.clone(),
                                        sender_peer_id: peer.clone(),
                                        ciphertext: data.to_vec(),
                                        signature: None,
                                        frame_type: 1,
                                        timestamp_unix_ms: chrono::Utc::now().timestamp_millis(),
                                        delivered: true,
                                    };
                                    let _ = warden_storage::MessageStore::store_message(db, msg);
                                }
                            }
                            Some(SessionEvent::Disconnected) => {
                                println!("Disconnected");
                                break;
                            }
                            None => break,
                        }
                    }
                    line = read_stdin_line() => {
                        match line {
                            Ok(line) => {
                                if line.trim().eq_ignore_ascii_case("/quit") {
                                    break;
                                }
                                if let Ok(ref db) = db {
                                    let msg = warden_storage::StoredMessage {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        conversation_id: conversation_id.clone(),
                                        sender_peer_id: String::new(),
                                        ciphertext: line.as_bytes().to_vec(),
                                        signature: None,
                                        frame_type: 1,
                                        timestamp_unix_ms: chrono::Utc::now().timestamp_millis(),
                                        delivered: true,
                                    };
                                    let _ = warden_storage::MessageStore::store_message(db, msg);
                                }
                                let _ = session.send(Bytes::from(line)).await;
                            }
                            Err(_) => break,
                        }
                    }
                }
            }
        }
        Command::Discovery { action } => match action {
            DiscoveryAction::Announce { port, bootstrap, relay } => {
                let keypair = warden_identity::IdentityKeypair::load(identity_path()?)?;
                let listen_addr: libp2p::Multiaddr =
                    format!("/ip4/0.0.0.0/tcp/{port}").parse().unwrap();
                let bootstrap_addrs = parse_multiaddrs(&bootstrap)?;
                let relay_addrs = parse_multiaddrs(&relay)?;

                let node = warden_discovery::DiscoveryNode::new(&keypair, vec![listen_addr]).await?;

                println!("PeerID: {}", node.peer_id());
                println!("Listening on DHT port {port}");

                for relay_addr in &relay_addrs {
                    println!("Connecting to relay {relay_addr}...");
                    node.connect_relay(relay_addr.clone()).await?;
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }

                if !bootstrap_addrs.is_empty() {
                    println!("Bootstrapping to {} peers...", bootstrap_addrs.len());
                    node.bootstrap(bootstrap_addrs).await?;
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }

                match node.advertised_relay_addrs().await {
                    Ok(addrs) if !addrs.is_empty() => {
                        println!("Relay address(es) available:");
                        for addr in &addrs {
                            println!("  {addr}");
                        }
                    }
                    _ => {}
                }

                println!("Announcing our address on the DHT...");
                node.announce().await?;
                println!("Announced! Keeping discovery node running. Press Ctrl+C to stop.");
                loop {
                    tokio::time::sleep(Duration::from_secs(3600)).await;
                }
            }
            DiscoveryAction::Resolve { peer_id, bootstrap } => {
                let keypair = warden_identity::IdentityKeypair::load(identity_path()?)?;
                let bootstrap_addrs = parse_multiaddrs(&bootstrap)?;

                let node = warden_discovery::DiscoveryNode::new(&keypair, vec![]).await?;

                if !bootstrap_addrs.is_empty() {
                    println!("Bootstrapping to {} peers...", bootstrap_addrs.len());
                    node.bootstrap(bootstrap_addrs).await?;
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }

                let target = warden_core::PeerId::new(peer_id);
                println!("Resolving {target} on DHT...");
                match node.resolve(target).await {
                    Ok(addrs) => {
                        println!("Found {} address(es):", addrs.len());
                        for addr in &addrs {
                            println!("  {addr}");
                        }
                    }
                    Err(e) => {
                        println!("Resolve failed: {e}");
                    }
                }
            }
            DiscoveryAction::RelayConnect { relay_addr, bootstrap } => {
                let keypair = warden_identity::IdentityKeypair::load(identity_path()?)?;
                let bootstrap_addrs = parse_multiaddrs(&bootstrap)?;
                let relay = relay_addr.parse::<libp2p::Multiaddr>()?;

                let node = warden_discovery::DiscoveryNode::new(&keypair, vec![]).await?;

                if !bootstrap_addrs.is_empty() {
                    println!("Bootstrapping to {} peers...", bootstrap_addrs.len());
                    node.bootstrap(bootstrap_addrs).await?;
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }

                println!("Connecting to relay {relay}...");
                node.connect_relay(relay).await?;
                tokio::time::sleep(Duration::from_secs(5)).await;

                match node.advertised_relay_addrs().await {
                    Ok(addrs) if !addrs.is_empty() => {
                        println!("Relay address(es) granted:");
                        for addr in &addrs {
                            println!("  {addr}");
                        }
                    }
                    Ok(_) => {
                        println!("No relay addresses granted yet (reservation may be pending).");
                    }
                    Err(e) => {
                        println!("Failed to get relay addresses: {e}");
                    }
                }

                println!("Relay node running. Press Ctrl+C to stop.");
                loop {
                    tokio::time::sleep(Duration::from_secs(3600)).await;
                }
            }
            DiscoveryAction::RelayAddrs => {
                let keypair = warden_identity::IdentityKeypair::load(identity_path()?)?;
                let node = warden_discovery::DiscoveryNode::new(&keypair, vec![]).await?;

                match node.advertised_relay_addrs().await {
                    Ok(addrs) if !addrs.is_empty() => {
                        println!("Relay address(es):");
                        for addr in &addrs {
                            println!("  {addr}");
                        }
                    }
                    Ok(_) => println!("No relay addresses available."),
                    Err(e) => println!("Failed to get relay addresses: {e}"),
                }
            }
        },
        Command::Contacts { action } => match action {
            ContactsAction::Add { peer_id, alias } => {
                let db = open_db()?;
                let contact = warden_storage::Contact {
                    peer_id,
                    public_key: Vec::new(),
                    alias,
                    added_at_ms: chrono::Utc::now().timestamp_millis(),
                    last_seen_ms: None,
                };
                warden_storage::ContactStore::add_contact(&db, contact)?;
                println!("Contact added.");
            }
            ContactsAction::List => {
                let db = open_db()?;
                let contacts = warden_storage::ContactStore::list_contacts(&db)?;
                if contacts.is_empty() {
                    println!("No contacts.");
                } else {
                    for c in &contacts {
                        let alias = c.alias.as_deref().unwrap_or("-");
                        let last_seen = c
                            .last_seen_ms
                            .map(format_since)
                            .unwrap_or_else(|| "never".into());
                        println!("  {:<15}  alias={:<20}  last_seen={last_seen}", c.peer_id, alias);
                    }
                }
            }
            ContactsAction::Remove { peer_id } => {
                let db = open_db()?;
                match warden_storage::ContactStore::remove_contact(&db, &peer_id) {
                    Ok(()) => println!("Contact removed."),
                    Err(e) => println!("Error: {e}"),
                }
            }
        },
        Command::History { peer_id, limit: _ } => {
            let db = open_db()?;
            let conv_id = conversation_id(&peer_id);
            match warden_storage::MessageStore::get_conversation(&db, &conv_id) {
                Ok(msgs) => {
                    if msgs.is_empty() {
                        println!("No messages with {peer_id}.");
                    } else {
                        for msg in &msgs {
                            let sender = if msg.sender_peer_id.is_empty() {
                                "me"
                            } else {
                                &msg.sender_peer_id
                            };
                            let ts = format_timestamp(msg.timestamp_unix_ms);
                            let text = String::from_utf8_lossy(&msg.ciphertext);
                            println!("[{ts}] {sender}: {text}");
                        }
                    }
                }
                Err(e) => println!("Error loading history: {e}"),
            }
        }
        Command::Daemon { ssh_port, dht_port, bootstrap, relay } => {
            let keypair = warden_identity::IdentityKeypair::load(identity_path()?)?;
            let db = open_db();

            // Auto-load known peers as bootstrap if none specified
            let mut bootstrap = bootstrap;
            if bootstrap.is_empty() {
                let known = load_known_peers();
                if !known.is_empty() {
                    println!("Auto-bootstrapping from {} known peer(s)", known.len());
                    bootstrap = known;
                }
            }

            let dht_listen: libp2p::Multiaddr =
                format!("/ip4/0.0.0.0/tcp/{dht_port}").parse().unwrap();
            let bootstrap_addrs = parse_multiaddrs(&bootstrap)?;
            let relay_addrs = parse_multiaddrs(&relay)?;

            let dht_node =
                warden_discovery::DiscoveryNode::new(&keypair, vec![dht_listen]).await?;

            for relay_addr in &relay_addrs {
                println!("Connecting to relay {relay_addr}...");
                let _ = dht_node.connect_relay(relay_addr.clone()).await;
                tokio::time::sleep(Duration::from_secs(2)).await;
            }

            if !bootstrap_addrs.is_empty() {
                println!("Bootstrapping DHT to {} peers...", bootstrap_addrs.len());
                let _ = dht_node.bootstrap(bootstrap_addrs).await;
                tokio::time::sleep(Duration::from_secs(3)).await;
            }

            // Add SSH address to DHT announcement so peers can resolve it
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            if let Ok(addrs) = dht_node.listening_addrs().await {
                for a in &addrs {
                    if let Some(sa) = multiaddr_to_socketaddr(a) {
                        let ssh_ma: libp2p::Multiaddr =
                            format!("/ip4/{}/tcp/{ssh_port}", sa.ip()).parse().unwrap();
                        let _ = dht_node.add_announce_addr(ssh_ma).await;
                        break;
                    }
                }
            }

            // Persist our listening addresses as known peers
            if let Ok(addrs) = dht_node.listening_addrs().await {
                let known: Vec<String> = addrs.iter().map(|a| a.to_string()).collect();
                save_known_peers(&known);
            }

            let _ = dht_node.announce().await;

            // Start SSH server
            println!("Starting SSH chat server on port {ssh_port}...");
            let (tx, mut rx) = mpsc::channel::<warden_transport::ChatSession>(64);

            let server_task = tokio::spawn(async move {
                if let Err(e) = warden_transport::start_server("0.0.0.0", ssh_port, tx).await {
                    tracing::error!("Server error: {e}");
                }
            });

            // Background outbox retry loop
            let dht_node_clone = dht_node.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(30)).await;
                    if let Ok(db) = open_db() {
                        let pending = match warden_storage::OutboxStore::all_pending(&db) {
                            Ok(p) => p,
                            Err(e) => {
                                tracing::warn!("Outbox retry: failed to query: {e}");
                                continue;
                            }
                        };
                        let mut peers: Vec<String> = pending
                            .iter()
                            .map(|e| e.target_peer_id.clone())
                            .collect();
                        peers.sort();
                        peers.dedup();
                        for peer_id in &peers {
                            let target = warden_core::PeerId::new(peer_id.as_str());
                            match dht_node_clone.resolve(target).await {
                                Ok(addrs) => {
                                    for addr in &addrs {
                                        tracing::info!("Outbox retry: trying to connect to {peer_id} via {addr}");
                                        let addr_str = multiaddr_to_socketaddr(addr)
                                            .map(|sa| sa.to_string())
                            .unwrap_or_else(|| addr.to_string());
                                        match connect_with_default_known_hosts(&addr_str).await {
                                            Ok(mut session) => {
                                                flush_outbox_for_peer(&mut session, peer_id).await;
                                                break;
                                            }
                                            Err(e) => {
                                                tracing::warn!("Outbox retry: failed to connect to {peer_id}: {e}");
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::debug!("Outbox retry: cannot resolve {peer_id}: {e}");
                                }
                            }
                        }
                    }
                }
            });

            println!("Daemon running. PeerID: {}", keypair.peer_id());
            println!("  SSH :0.0.0.0:{ssh_port}");
            println!("  DHT :0.0.0.0:{dht_port}");
            println!("Press Ctrl+C to stop.");

            // Accept incoming sessions
            while let Some(mut session) = rx.recv().await {
                let peer = session.peer_id.clone();
                if let Ok(ref db) = db {
                    let _ = warden_storage::ContactStore::update_last_seen(
                        db,
                        &peer,
                        chrono::Utc::now().timestamp_millis(),
                    );
                }
                println!("New session from {peer}");
                tokio::spawn(async move {
                    flush_outbox_for_peer(&mut session, &peer).await;
                    let db_task = open_db();
                    let conversation_id = conversation_id(&peer);
                    loop {
                        tokio::select! {
                            event = session.recv() => {
                                match event {
                                    Some(SessionEvent::Message(data)) => {
                                        let text = String::from_utf8_lossy(&data);
                                        println!("[{peer}] {text}");
                                        if let Ok(ref db) = db_task {
                                            let msg = warden_storage::StoredMessage {
                                                id: uuid::Uuid::new_v4().to_string(),
                                                conversation_id: conversation_id.clone(),
                                                sender_peer_id: peer.clone(),
                                                ciphertext: data.to_vec(),
                                                signature: None,
                                                frame_type: 1,
                                                timestamp_unix_ms: chrono::Utc::now().timestamp_millis(),
                                                delivered: true,
                                            };
                                            let _ = warden_storage::MessageStore::store_message(db, msg);
                                        }
                                    }
                                    Some(SessionEvent::Disconnected) => {
                                        println!("[{peer}] disconnected");
                                        break;
                                    }
                                    None => break,
                                }
                            }
                            line = read_stdin_line() => {
                                match line {
                                    Ok(line) => {
                                        if line.trim().eq_ignore_ascii_case("/quit") {
                                            break;
                                        }
                                        if let Ok(ref db) = db_task {
                                            let msg = warden_storage::StoredMessage {
                                                id: uuid::Uuid::new_v4().to_string(),
                                                conversation_id: conversation_id.clone(),
                                                sender_peer_id: String::new(),
                                                ciphertext: line.as_bytes().to_vec(),
                                                signature: None,
                                                frame_type: 1,
                                                timestamp_unix_ms: chrono::Utc::now().timestamp_millis(),
                                                delivered: true,
                                            };
                                            let _ = warden_storage::MessageStore::store_message(db, msg);
                                        }
                                        let _ = session.send(Bytes::from(line)).await;
                                    }
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                });
            }

            let _ = server_task.await;
        }
        Command::Tui { port, dht_port, bootstrap, relay } => {
            let keypair = warden_identity::IdentityKeypair::load(identity_path()?)?;
            let db = open_db()?;

            // Auto-load known peers as bootstrap if none specified
            let mut bootstrap = bootstrap;
            if bootstrap.is_empty() {
                let known = load_known_peers();
                if !known.is_empty() {
                    println!("Auto-bootstrapping from {} known peer(s)", known.len());
                    bootstrap = known;
                }
            }
            let bootstrap_addrs = parse_multiaddrs(&bootstrap)?;
            let relay_addrs = parse_multiaddrs(&relay)?;

            // Start DHT node
            let dht_listen: libp2p::Multiaddr =
                format!("/ip4/0.0.0.0/tcp/{dht_port}").parse().unwrap();
            let dht_node = warden_discovery::DiscoveryNode::new(&keypair, vec![dht_listen]).await?;

            for relay_addr in &relay_addrs {
                println!("Connecting to relay {relay_addr}...");
                let _ = dht_node.connect_relay(relay_addr.clone()).await;
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }

            if !bootstrap_addrs.is_empty() {
                println!("Bootstrapping DHT to {} peers...", bootstrap_addrs.len());
                let _ = dht_node.bootstrap(bootstrap_addrs).await;
                tokio::time::sleep(std::time::Duration::from_secs(4)).await;
                let _ = dht_node.retry_bootstrap().await;
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }

            // Announce PeerID + SSH address on DHT
            if let Ok(addrs) = dht_node.listening_addrs().await {
                for a in &addrs {
                    if let Some(sa) = multiaddr_to_socketaddr(a) {
                        let ssh_ma: libp2p::Multiaddr =
                            format!("/ip4/{}/tcp/{port}", sa.ip()).parse().unwrap();
                        let _ = dht_node.add_announce_addr(ssh_ma).await;
                        break;
                    }
                }
            }
            // Persist our listening addresses as known peers
            if let Ok(addrs) = dht_node.listening_addrs().await {
                let known: Vec<String> = addrs.iter().map(|a| a.to_string()).collect();
                save_known_peers(&known);
            }

            let _ = dht_node.announce().await;
            println!("DHT node ready. PeerID: {}", keypair.peer_id());

            // Start SSH server
            let (tx, rx) = mpsc::channel::<warden_transport::ChatSession>(64);
            let server_task = tokio::spawn(async move {
                if let Err(e) = warden_transport::start_server("0.0.0.0", port, tx).await {
                    eprintln!("TUI server error: {e}");
                }
            });

            let mut app = warden_tui::TuiApp::new(db, rx, Some(dht_node), port);
            app.run().await?;
            server_task.abort();
        }
        Command::Outbox { action } => match action {
            OutboxAction::Send { peer_id, message } => {
                let db = open_db()?;
                let entry = warden_storage::OutboxEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    target_peer_id: peer_id.clone(),
                    frame_bytes: message.into_bytes(),
                    created_at_ms: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as i64,
                    retry_count: 0,
                    last_attempt_ms: None,
                    delivered: false,
                };
                warden_storage::OutboxStore::enqueue(&db, entry)?;
                println!("Message queued for outbox delivery to {peer_id}.");
            }
            OutboxAction::Flush => {
                let db = open_db()?;
                let pending = warden_storage::OutboxStore::all_pending(&db)?;
                if pending.is_empty() {
                    println!("No pending outbox messages.");
                } else {
                    println!("{} pending message(s). Flush is automatic on connect.", pending.len());
                }
            }
            OutboxAction::List => {
                let db = open_db()?;
                let pending = warden_storage::OutboxStore::all_pending(&db)?;
                if pending.is_empty() {
                    println!("No pending outbox messages.");
                } else {
                    for entry in &pending {
                        println!("  {} -> {} (retries: {}, created: {})",
                            entry.id, entry.target_peer_id, entry.retry_count, entry.created_at_ms);
                    }
                }
            }
        },
        Command::Groups { action } => match action {
            GroupsAction::Create { name, members } => {
                let db = open_db()?;
                let id = uuid::Uuid::new_v4().to_string();
                warden_storage::GroupStore::create_group(&db, &id, &name)?;
                for m in &members {
                    warden_storage::GroupStore::add_member(&db, &id, m)?;
                }
                println!("Group '{name}' created with id: {id}");
                println!("Members: {}", members.join(", "));
            }
            GroupsAction::List => {
                let db = open_db()?;
                let groups = warden_storage::GroupStore::list_groups(&db)?;
                if groups.is_empty() {
                    println!("No groups.");
                } else {
                    for g in &groups {
                        let members = warden_storage::GroupStore::group_members(&db, &g.id)
                            .unwrap_or_default();
                        println!("  {:<36}  name={:<20}  members={}", g.id, g.name, members.len());
                    }
                }
            }
            GroupsAction::Members { group_id } => {
                let db = match open_db() {
                    Ok(db) => db,
                    Err(e) => {
                        println!("Error opening database: {e}");
                        return Ok(());
                    }
                };
                let group = match warden_storage::GroupStore::get_group(&db, &group_id) {
                    Ok(g) => g,
                    Err(e) => {
                        println!("Group not found: {e}");
                        return Ok(());
                    }
                };
                println!("Group: {} ({})", group.name, group.id);
                let members = match warden_storage::GroupStore::group_members(&db, &group_id) {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Error loading members: {e}");
                        return Ok(());
                    }
                };
                for m in &members {
                    println!("  {:<15}  added={}", m.peer_id, format_timestamp(m.added_at_ms));
                }
            }
            GroupsAction::Send { group_id, message } => {
                let db = match open_db() {
                    Ok(db) => db,
                    Err(e) => {
                        println!("Error opening database: {e}");
                        return Ok(());
                    }
                };
                let group = match warden_storage::GroupStore::get_group(&db, &group_id) {
                    Ok(g) => g,
                    Err(e) => {
                        println!("Group not found: {e}");
                        return Ok(());
                    }
                };
                let members = match warden_storage::GroupStore::group_members(&db, &group_id) {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Error loading members: {e}");
                        return Ok(());
                    }
                };
                println!("Sending to {} members in group '{}'...", members.len(), group.name);

                let now_ms = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as i64;

                // Store the message locally
                let msg = warden_storage::StoredMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id: group_id.clone(),
                    sender_peer_id: String::new(),
                    ciphertext: message.as_bytes().to_vec(),
                    signature: None,
                    frame_type: 1,
                    timestamp_unix_ms: now_ms,
                    delivered: false,
                };
                let _ = warden_storage::MessageStore::store_message(&db, msg);

                // Fanout to each member
                let mut delivered = 0;
                for m in &members {
                    let addr = if let Ok(sa) = m.peer_id.parse::<std::net::SocketAddr>() {
                        sa.to_string()
                    } else {
                        match resolve_peer_id(&m.peer_id, &[], &[]).await {
                            Ok(sa) => sa.to_string(),
                            Err(e) => {
                                println!("  Could not resolve {}: {e}", m.peer_id);
                                // Queue to outbox for offline delivery
                                let outbox_msg = warden_storage::StoredMessage {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    conversation_id: group_id.clone(),
                                    sender_peer_id: String::new(),
                                    ciphertext: message.as_bytes().to_vec(),
                                    signature: None,
                                    frame_type: 1,
                                    timestamp_unix_ms: now_ms,
                                    delivered: false,
                                };
                                let _ = warden_storage::MessageStore::store_message(&db, outbox_msg);
                                let entry = warden_storage::OutboxEntry {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    target_peer_id: m.peer_id.clone(),
                                    frame_bytes: message.as_bytes().to_vec(),
                                    created_at_ms: now_ms,
                                    retry_count: 0,
                                    last_attempt_ms: None,
                                    delivered: false,
                                };
                                let _ = warden_storage::OutboxStore::enqueue(&db, entry);
                                println!("  Queued for offline delivery to {}", m.peer_id);
                                continue;
                            }
                        }
                    };
                    match connect_with_default_known_hosts(&addr).await {
                        Ok(mut session) => {
                            if session.send(Bytes::from(message.clone())).await.is_ok() {
                                delivered += 1;
                                println!("  Delivered to {}", m.peer_id);
                            }
                        }
                        Err(_) => {
                            // Queue to outbox for offline delivery
                            let outbox_msg = warden_storage::StoredMessage {
                                id: uuid::Uuid::new_v4().to_string(),
                                conversation_id: group_id.clone(),
                                sender_peer_id: String::new(),
                                ciphertext: message.as_bytes().to_vec(),
                                signature: None,
                                frame_type: 1,
                                timestamp_unix_ms: now_ms,
                                delivered: false,
                            };
                            let _ = warden_storage::MessageStore::store_message(&db, outbox_msg);
                            let entry = warden_storage::OutboxEntry {
                                id: uuid::Uuid::new_v4().to_string(),
                                target_peer_id: m.peer_id.clone(),
                                frame_bytes: message.as_bytes().to_vec(),
                                created_at_ms: now_ms,
                                retry_count: 0,
                                last_attempt_ms: None,
                                delivered: false,
                            };
                            let _ = warden_storage::OutboxStore::enqueue(&db, entry);
                            println!("  Queued for offline delivery to {}", m.peer_id);
                        }
                    }
                }
                println!("Sent to {delivered}/{} members.", members.len());
            }
        },
    }

    Ok(())
}

/// Flush pending outbox messages for a peer after a connection is established.
async fn flush_outbox_for_peer(
    session: &mut warden_transport::ChatSession,
    peer_id: &str,
) {
    let db = match open_db() {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to open DB for outbox flush: {e}");
            return;
        }
    };
    let entries = match warden_storage::OutboxStore::pending_for_peer(&db, peer_id) {
        Ok(e) => e,
        Err(e) => {
            tracing::error!("Failed to query outbox for {peer_id}: {e}");
            return;
        }
    };
    if entries.is_empty() {
        return;
    }
    for entry in &entries {
        match session.send(Bytes::from(entry.frame_bytes.clone())).await {
            Ok(()) => {
                if let Ok(db) = open_db() {
                    let _ = warden_storage::OutboxStore::mark_delivered_outbox(&db, &entry.id);
                }
                println!("  Delivered pending message {}", entry.id);
            }
            Err(e) => {
                tracing::error!("Failed to send outbox entry {}: {e}", entry.id);
                if let Ok(db) = open_db() {
                    let _ = warden_storage::OutboxStore::increment_retry(
                        &db,
                        &entry.id,
                        SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as i64,
                    );
                }
            }
        }
    }
}

async fn read_stdin_line() -> Result<String, std::io::Error> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    lines.next_line().await.map(|opt| opt.unwrap_or_default())
}

fn parse_multiaddrs(strings: &[String]) -> Result<Vec<libp2p::Multiaddr>, anyhow::Error> {
    strings
        .iter()
        .map(|s| Ok(s.parse::<libp2p::Multiaddr>()?))
        .collect()
}

fn conversation_id(peer_id: &str) -> String {
    peer_id.to_string()
}

fn format_since(timestamp_ms: i64) -> String {
    let now = chrono::Utc::now().timestamp_millis();
    let diff = now - timestamp_ms;
    if diff < 60_000 {
        "just now".into()
    } else if diff < 3_600_000 {
        format!("{}m ago", diff / 60_000)
    } else if diff < 86_400_000 {
        format!("{}h ago", diff / 3_600_000)
    } else {
        format!("{}d ago", diff / 86_400_000)
    }
}

fn format_timestamp(timestamp_ms: i64) -> String {
    let secs = timestamp_ms / 1000;
    let nanos = ((timestamp_ms % 1000) * 1_000_000) as u32;
    match chrono::DateTime::from_timestamp(secs, nanos) {
        Some(dt) => dt.format("%H:%M:%S").to_string(),
        None => "?".into(),
    }
}

#[cfg(test)]
mod tests {
    use clap::{CommandFactory, Parser};

    #[test]
    fn cli_parses_identity_init() {
        let cli = super::Cli::try_parse_from(["warden", "identity", "init"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_identity_show() {
        let cli = super::Cli::try_parse_from(["warden", "identity", "show"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_listen_defaults() {
        let cli = super::Cli::try_parse_from(["warden", "listen"]).unwrap();
        if let super::Command::Listen { port } = cli.command {
            assert_eq!(port, 2222);
        } else {
            panic!("expected Listen command");
        }
    }

    #[test]
    fn cli_parses_listen_custom_port() {
        let cli = super::Cli::try_parse_from(["warden", "listen", "--port", "3333"]).unwrap();
        if let super::Command::Listen { port } = cli.command {
            assert_eq!(port, 3333);
        } else {
            panic!("expected Listen command");
        }
    }

    #[test]
    fn cli_parses_connect() {
        let cli = super::Cli::try_parse_from(["warden", "connect", "127.0.0.1:2222"]).unwrap();
        if let super::Command::Connect { addr, .. } = cli.command {
            assert_eq!(addr, "127.0.0.1:2222");
        } else {
            panic!("expected Connect command");
        }
    }

    #[test]
    fn cli_parses_contacts_list() {
        let cli = super::Cli::try_parse_from(["warden", "contacts", "list"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_contacts_add() {
        let cli =
            super::Cli::try_parse_from(["warden", "contacts", "add", "peer123", "--alias", "alice"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_history() {
        let cli = super::Cli::try_parse_from(["warden", "history", "peer123"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_daemon_defaults() {
        let cli = super::Cli::try_parse_from(["warden", "daemon"]).unwrap();
        if let super::Command::Daemon { ssh_port, dht_port, .. } = cli.command {
            assert_eq!(ssh_port, 2222);
            assert_eq!(dht_port, 3333);
        } else {
            panic!("expected Daemon command");
        }
    }

    #[test]
    fn cli_parses_discovery_announce() {
        let cli = super::Cli::try_parse_from([
            "warden",
            "discovery",
            "announce",
            "--bootstrap",
            "/ip4/1.2.3.4/tcp/4001",
        ]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_outbox_send() {
        let cli = super::Cli::try_parse_from(["warden", "outbox", "send", "peer123", "hello"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_groups_create() {
        let cli =
            super::Cli::try_parse_from(["warden", "groups", "create", "mygroup", "peer1", "peer2"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn cli_parses_tui_defaults() {
        let cli = super::Cli::try_parse_from(["warden", "tui"]).unwrap();
        if let super::Command::Tui { port, dht_port, .. } = cli.command {
            assert_eq!(port, 2222);
            assert_eq!(dht_port, 0);
        } else {
            panic!("expected Tui command");
        }
    }

    #[test]
    fn cli_verify_app_name() {
        let cmd = super::Cli::command();
        assert_eq!(cmd.get_name(), "warden");
    }
}
