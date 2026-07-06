//! Terminal UI for Warden powered by `ratatui` and `crossterm`.
//!
//! Provides a split-pane chat interface: conversation sidebar, message
//! view, and input bar. [`TuiApp`] manages the event loop, key handling,
//! network polling, and rendering.

use std::time::{Duration, SystemTime};

use bytes::Bytes;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Terminal;
use tokio::sync::mpsc;
use warden_storage::{ContactStore, Database, MessageStore, OutboxEntry, OutboxStore, StoredMessage, GroupStore};
use warden_transport::{ChatSession, SessionEvent};

#[derive(Clone, PartialEq)]
enum ConvKind {
    Peer,
    Group,
}

struct Conversation {
    peer_id: String,
    kind: ConvKind,
    name: String,
    messages: Vec<StoredMessage>,
    session: Option<ChatSession>,
}

const COMMANDS: &[&str] = &[
    "/connect <addr>",
    "/group create <name>",
    "/group add <peer_id>",
    "/group list",
    "/quit",
];

pub struct TuiApp {
    db: Database,
    conversations: Vec<Conversation>,
    active_idx: usize,
    input: String,
    status: String,
    running: bool,
    session_rx: mpsc::Receiver<ChatSession>,
    connect_rx: mpsc::UnboundedReceiver<ChatSession>,
    connect_handle: mpsc::UnboundedSender<String>,
    show_cmd_palette: bool,
    cmd_palette_idx: usize,
}

impl TuiApp {
    pub fn new(db: Database, session_rx: mpsc::Receiver<ChatSession>) -> Self {
        let (connect_cmd_tx, _) = mpsc::unbounded_channel::<String>();
        let (_, connect_res_rx) = mpsc::unbounded_channel::<ChatSession>();
        Self {
            conversations: Vec::new(),
            active_idx: 0,
            input: String::new(),
            status: "Type / to see all commands | j/k: nav | Enter: send".into(),
            running: true,
            session_rx,
            db,
            connect_rx: connect_res_rx,
            connect_handle: connect_cmd_tx,
            show_cmd_palette: false,
            cmd_palette_idx: 0,
        }
    }

    fn start_connect_handler(&mut self) {
        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<String>();
        let (res_tx, res_rx) = mpsc::unbounded_channel::<ChatSession>();
        self.connect_handle = cmd_tx;
        self.connect_rx = res_rx;
        tokio::spawn(async move {
            while let Some(addr) = cmd_rx.recv().await {
                match warden_transport::connect(&addr).await {
                    Ok(session) => {
                        let _ = res_tx.send(session);
                    }
                    Err(e) => {
                        tracing::warn!("TUI connect to {addr} failed: {e}");
                    }
                }
            }
        });
    }

    pub fn connect_handle(&self) -> mpsc::UnboundedSender<String> {
        self.connect_handle.clone()
    }

    fn active_peer(&self) -> Option<&str> {
        self.conversations.get(self.active_idx).map(|c| c.peer_id.as_str())
    }

    fn active_kind(&self) -> ConvKind {
        self.conversations.get(self.active_idx).map(|c| &c.kind).cloned().unwrap_or(ConvKind::Peer)
    }

    fn active_name(&self) -> Option<&str> {
        self.conversations.get(self.active_idx).map(|c| c.name.as_str())
    }

    fn conv_label(&self, conv: &Conversation) -> String {
        let is_online = conv.session.is_some();
        let indicator = if is_online { "●" } else { "○" };
        let glyph = if conv.kind == ConvKind::Group { "#" } else { "@" };
        format!("{indicator} {glyph} {}", conv.name)
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.start_connect_handler();
        enable_raw_mode()?;
        std::io::stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(std::io::stdout());
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(100);
        let mut last_tick = SystemTime::now();

        while self.running {
            self.poll_network();

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed().unwrap_or_default())
                .unwrap_or(Duration::ZERO);

            if event::poll(timeout)? {
                let ev = event::read()?;
                self.handle_event(ev);
            }

            if last_tick.elapsed().unwrap_or_default() >= tick_rate {
                last_tick = SystemTime::now();
            }

            terminal.draw(|f| self.render(f))?;
        }

        disable_raw_mode()?;
        std::io::stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn poll_network(&mut self) {
        while let Ok(session) = self.session_rx.try_recv() {
            let peer = session.peer_id.clone();
            let _ = ContactStore::update_last_seen(&self.db, &peer, now_ms());
            if let Some(conv) = self.conversations.iter_mut().find(|c| c.peer_id == peer) {
                conv.session = Some(session);
            } else {
                let conv_id = conversation_id(&peer);
                let msgs = MessageStore::get_conversation(&self.db, &conv_id).unwrap_or_default();
                self.conversations.push(Conversation {
                    peer_id: peer.clone(),
                    kind: ConvKind::Peer,
                    name: peer.clone(),
                    messages: msgs,
                    session: Some(session),
                });
            }
            if self.db.list_contacts().unwrap_or_default().iter().all(|c| c.peer_id != peer) {
                let contact = warden_storage::Contact {
                    peer_id: peer.clone(),
                    public_key: Vec::new(),
                    alias: None,
                    added_at_ms: now_ms(),
                    last_seen_ms: Some(now_ms()),
                };
                let _ = ContactStore::add_contact(&self.db, contact);
            }
            self.status = format!("Connected: {peer}");
        }

            let to_remove: Vec<String> = self
            .conversations
            .iter_mut()
            .filter_map(|conv| {
                let session = conv.session.as_mut()?;
                loop {
                    match session.receiver.try_recv() {
                        Ok(SessionEvent::Message(data)) => {
                            let msg = StoredMessage {
                                id: uuid::Uuid::new_v4().to_string(),
                                conversation_id: conversation_id(&conv.peer_id),
                                sender_peer_id: conv.peer_id.clone(),
                                ciphertext: data.to_vec(),
                                signature: None,
                                frame_type: 1,
                                timestamp_unix_ms: now_ms(),
                                delivered: true,
                            };
                            let _ = MessageStore::store_message(&self.db, msg.clone());
                            conv.messages.push(msg);
                        }
                        Ok(SessionEvent::Disconnected) => {
                            self.status = format!("{} disconnected", conv.peer_id);
                            return Some(conv.peer_id.clone());
                        }
                        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => break,
                        Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                            self.status = format!("{} session closed", conv.peer_id);
                            return Some(conv.peer_id.clone());
                        }
                    }
                }
                None
            })
            .collect();

        for peer_id in to_remove {
            self.conversations.retain(|c| c.peer_id != peer_id);
        }
        if self.active_idx >= self.conversations.len() && !self.conversations.is_empty() {
            self.active_idx = self.conversations.len() - 1;
        } else if self.conversations.is_empty() {
            self.active_idx = 0;
        }
        // Handle outbound connect results
        while let Ok(session) = self.connect_rx.try_recv() {
            let peer = session.peer_id.clone();
            if !self.conversations.iter().any(|c| c.peer_id == peer) {
                let conv_id = conversation_id(&peer);
                let msgs = MessageStore::get_conversation(&self.db, &conv_id).unwrap_or_default();
                self.conversations.push(Conversation {
                    peer_id: peer.clone(),
                    kind: ConvKind::Peer,
                    name: peer.clone(),
                    messages: msgs,
                    session: Some(session),
                });
                self.status = format!("Connected to {peer}");
            }
        }

        let online = self.conversations.iter().filter(|c| c.session.is_some()).count();
        let total = self.conversations.len();
        self.status = format!("{online}/{total} online | /quit: exit | j/k: nav | Enter: send");

        // Load groups into sidebar if not present
        let known_groups = self.db.list_groups().unwrap_or_default();
        for group in &known_groups {
            let gid = format!("group:{}", group.id);
            if !self.conversations.iter().any(|c| c.peer_id == gid) {
                self.conversations.push(Conversation {
                    peer_id: gid,
                    kind: ConvKind::Group,
                    name: group.name.clone(),
                    messages: Vec::new(),
                    session: None,
                });
            }
        }
    }

    fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if self.show_cmd_palette {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            self.cmd_palette_idx = (self.cmd_palette_idx + 1) % COMMANDS.len();
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            self.cmd_palette_idx = self.cmd_palette_idx.wrapping_sub(1).min(COMMANDS.len() - 1);
                        }
                        KeyCode::Enter => {
                            let cmd = COMMANDS[self.cmd_palette_idx];
                            self.input = cmd.split(' ').next().unwrap_or(cmd).to_string();
                            self.show_cmd_palette = false;
                            self.cmd_palette_idx = 0;
                        }
                        KeyCode::Esc => {
                            self.show_cmd_palette = false;
                            self.cmd_palette_idx = 0;
                        }
                        _ => {}
                    }
                    return;
                }
                match key.code {
                    KeyCode::Char('q') if self.input.is_empty() => self.running = false,
                    KeyCode::Up | KeyCode::Char('k') if self.input.is_empty() => self.prev_conv(),
                    KeyCode::Down | KeyCode::Char('j') if self.input.is_empty() => self.next_conv(),
                    KeyCode::Enter => self.send(),
                    KeyCode::Backspace => { self.input.pop(); }
                    KeyCode::Esc => self.input.clear(),
                    KeyCode::Char('/') if self.input.is_empty() => {
                        self.show_cmd_palette = true;
                        self.cmd_palette_idx = 0;
                    }
                    KeyCode::Char(c) => self.input.push(c),
                    _ => {}
                }
            }
            Event::Resize(_, _) => {}
            _ => {}
        }
    }

    fn prev_conv(&mut self) {
        if !self.conversations.is_empty() {
            self.active_idx = self.active_idx.wrapping_sub(1).min(self.conversations.len() - 1);
        }
    }

    fn next_conv(&mut self) {
        if !self.conversations.is_empty() {
            self.active_idx = (self.active_idx + 1) % self.conversations.len();
        }
    }

    fn handle_command(&mut self, text: &str) -> bool {
        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }
        match parts[0] {
            "/quit" => {
                self.running = false;
                true
            }
            "/connect" => {
                if parts.len() < 2 {
                    self.status = "Usage: /connect <ip:port>".into();
                } else {
                    let addr = parts[1..].join(" ");
                    self.connect_handle.send(addr.clone()).ok();
                    self.status = format!("Connecting to {addr}...");
                }
                true
            }
            "/group" => {
                if parts.len() < 2 {
                    self.status = "Usage: /group create <name> | /group add <peer_id> | /group list".into();
                    return true;
                }
                match parts[1] {
                    "create" => {
                        if parts.len() < 3 {
                            self.status = "Usage: /group create <name>".into();
                        } else {
                            let name = parts[2..].join(" ");
                            let id = uuid::Uuid::new_v4().to_string();
                            if self.db.create_group(&id, &name).is_ok() {
                                let gid = format!("group:{id}");
                                self.conversations.push(Conversation {
                                    peer_id: gid,
                                    kind: ConvKind::Group,
                                    name: name.clone(),
                                    messages: Vec::new(),
                                    session: None,
                                });
                                self.status = format!("Group '{name}' created");
                            } else {
                                self.status = "Failed to create group".into();
                            }
                        }
                    }
                    "add" => {
                        if parts.len() < 3 {
                            self.status = "Usage: /group add <peer_id>".into();
                        } else {
                            let peer_id = parts[2];
                            if let Some(conv) = self.conversations.get(self.active_idx) {
                                if conv.kind == ConvKind::Group {
                                    let group_id = conv.peer_id.strip_prefix("group:").unwrap_or(&conv.peer_id).to_string();
                                    if self.db.add_member(&group_id, peer_id).is_ok() {
                                        self.status = format!("Added {peer_id} to group");
                                    } else {
                                        self.status = "Failed to add member".into();
                                    }
                                } else {
                                    self.status = "Select a group conversation first".into();
                                }
                            }
                        }
                    }
                    "list" => {
                        let groups = self.db.list_groups().unwrap_or_default();
                        if groups.is_empty() {
                            self.status = "No groups".into();
                        } else {
                            let names: Vec<String> = groups.iter().map(|g| g.name.clone()).collect();
                            self.status = format!("Groups: {}", names.join(", "));
                        }
                    }
                    _ => {
                        self.status = "Unknown /group subcommand: create | add | list".into();
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn send(&mut self) {
        let text = self.input.trim().to_string();
        if text.is_empty() {
            return;
        }
        if text.starts_with('/') {
            self.handle_command(&text);
            self.input.clear();
            return;
        }

        let target = match self.active_peer() {
            Some(p) => p.to_string(),
            None => {
                self.status = "No conversation selected".into();
                self.input.clear();
                return;
            }
        };

        let now = now_ms();
        let kind = self.active_kind();

        if kind == ConvKind::Group {
            let group_id = target.strip_prefix("group:").unwrap_or(&target).to_string();
            let members = self.db.group_members(&group_id).unwrap_or_default();
            let mut delivered_count = 0;

            for m in &members {
                let cid = conversation_id(&m.peer_id);
                let msg = StoredMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id: cid,
                    sender_peer_id: String::new(),
                    ciphertext: text.as_bytes().to_vec(),
                    signature: None,
                    frame_type: 1,
                    timestamp_unix_ms: now,
                    delivered: true,
                };
                let _ = MessageStore::store_message(&self.db, msg);

                if let Some(peer_conv) =
                    self.conversations.iter_mut().find(|c| c.peer_id == m.peer_id && c.kind == ConvKind::Peer) &&
                    let Some(ref mut s) = peer_conv.session
                {
                    let _ = s.sender.try_send(Bytes::from(text.clone()));
                    delivered_count += 1;
                }
            }

            if let Some(conv) = self.conversations.get_mut(self.active_idx) {
                conv.messages.push(StoredMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id: format!("group:{group_id}"),
                    sender_peer_id: String::new(),
                    ciphertext: text.as_bytes().to_vec(),
                    signature: None,
                    frame_type: 1,
                    timestamp_unix_ms: now,
                    delivered: true,
                });
            }

            self.status = format!("Group: delivered to {delivered_count}/{} members", members.len());
        } else {
            let msg = StoredMessage {
                id: uuid::Uuid::new_v4().to_string(),
                conversation_id: conversation_id(&target),
                sender_peer_id: String::new(),
                ciphertext: text.as_bytes().to_vec(),
                signature: None,
                frame_type: 1,
                timestamp_unix_ms: now,
                delivered: true,
            };
            let _ = MessageStore::store_message(&self.db, msg);

            if let Some(conv) = self.conversations.get_mut(self.active_idx) {
                let stored = StoredMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id: conversation_id(&target),
                    sender_peer_id: String::new(),
                    ciphertext: text.as_bytes().to_vec(),
                    signature: None,
                    frame_type: 1,
                    timestamp_unix_ms: now,
                    delivered: true,
                };
                conv.messages.push(stored);

                if let Some(ref mut session) = conv.session {
                    let _ = session.sender.try_send(Bytes::from(text));
                } else {
                    let entry = OutboxEntry {
                        id: uuid::Uuid::new_v4().to_string(),
                        target_peer_id: target,
                        frame_bytes: text.into_bytes(),
                        created_at_ms: now,
                        retry_count: 0,
                        last_attempt_ms: None,
                        delivered: false,
                    };
                    let _ = OutboxStore::enqueue(&self.db, entry);
                    self.status = "Queued for delivery".into();
                }
            }
        }

        self.input.clear();
    }

    fn render(&self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(area);

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(30),
                Constraint::Min(10),
            ])
            .split(chunks[0]);

        self.render_sidebar(frame, main[0]);
        self.render_messages(frame, main[1]);
        self.render_input(frame, chunks[1]);
        self.render_status(frame, chunks[2]);

        if self.show_cmd_palette {
            self.render_cmd_palette(frame, chunks[1]);
        }
    }

    fn render_cmd_palette(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let height = COMMANDS.len() as u16 + 2;
        let palette_area = ratatui::layout::Rect {
            x: area.x + 1,
            y: area.y.saturating_sub(height).max(0),
            width: 40.min(area.width.saturating_sub(2)),
            height,
        };
        let items: Vec<ListItem> = COMMANDS
            .iter()
            .enumerate()
            .map(|(i, cmd)| {
                let style = if i == self.cmd_palette_idx {
                    Style::default().fg(Color::Yellow).bg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Cyan)
                };
                let prefix = if i == self.cmd_palette_idx { ">" } else { " " };
                ListItem::new(format!("{prefix} {cmd}")).style(style)
            })
            .collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Commands"))
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_widget(list, palette_area);
    }

    fn render_sidebar(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let items: Vec<ListItem> = self
            .conversations
            .iter()
            .enumerate()
            .map(|(i, conv)| {
                let prefix = if i == self.active_idx { ">" } else { " " };
                let style = if i == self.active_idx {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{prefix} {}", self.conv_label(conv))).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Chats"))
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_widget(list, area);
    }

    fn render_messages(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let peer_label = self.active_name().or_else(|| self.active_peer()).unwrap_or("(no selection)");

        let lines: Vec<Line> = if let Some(conv) = self.conversations.get(self.active_idx) {
            conv.messages
                .iter()
                .map(|msg| {
                    let sender = if msg.sender_peer_id.is_empty() { "me" } else { "them" };
                    let ts = format_timestamp(msg.timestamp_unix_ms);
                    let text = String::from_utf8_lossy(&msg.ciphertext);
                    Line::from(vec![
                        Span::raw(format!("[{ts}] ")),
                        Span::styled(
                            format!("{sender}: "),
                            Style::default().fg(if sender == "me" { Color::Cyan } else { Color::Green }),
                        ),
                        Span::raw(text.to_string()),
                    ])
                })
                .collect()
        } else {
            vec![Line::from("No messages")]
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Chat — {peer_label}"));
        let para = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(para, area);
    }

    fn render_input(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let block = Block::default().borders(Borders::ALL).title("Message");
        let para = Paragraph::new(self.input.as_str())
            .block(block)
            .style(Style::default().fg(Color::White));
        frame.render_widget(para, area);
    }

    fn render_status(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let para = Paragraph::new(self.status.as_str())
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(para, area);
    }
}

fn conversation_id(peer_id: &str) -> String {
    peer_id.to_string()
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

fn format_timestamp(ms: i64) -> String {
    let secs = ms / 1000;
    let nanos = ((ms % 1000) * 1_000_000) as u32;
    match chrono::DateTime::from_timestamp(secs, nanos) {
        Some(dt) => dt.format("%H:%M").to_string(),
        None => "?".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use tokio::sync::mpsc;
    use warden_storage::Database;

    fn test_db() -> Database {
        let db = Database::open(":memory:").unwrap();
        db.migrate().unwrap();
        db
    }

    fn key_event(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new(code, KeyModifiers::empty()))
    }

    #[test]
    fn now_ms_returns_positive() {
        assert!(now_ms() > 1_700_000_000_000i64);
    }

    #[test]
    fn conversation_id_matches_input() {
        assert_eq!(conversation_id("alice"), "alice");
        assert_eq!(conversation_id("bob"), "bob");
    }

    #[test]
    fn format_timestamp_valid() {
        let s = format_timestamp(1_700_000_000_000i64);
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn format_timestamp_invalid() {
        let s = format_timestamp(i64::MAX);
        assert_eq!(s, "?");
    }

    #[test]
    fn tui_app_new_defaults() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let app = TuiApp::new(db, rx);
        assert!(app.conversations.is_empty());
        assert_eq!(app.active_idx, 0);
        assert!(app.input.is_empty());
        assert_eq!(app.status, "Type / to see all commands | j/k: nav | Enter: send");
        assert!(app.running);
    }

    #[test]
    fn key_q_quits_when_input_empty() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.handle_event(key_event(KeyCode::Char('q')));
        assert!(!app.running);
    }

    #[test]
    fn key_char_appends_to_input() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.handle_event(key_event(KeyCode::Char('h')));
        app.handle_event(key_event(KeyCode::Char('i')));
        assert_eq!(app.input, "hi");
    }

    #[test]
    fn key_q_does_not_quit_with_input() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.handle_event(key_event(KeyCode::Char('x')));
        app.handle_event(key_event(KeyCode::Char('q')));
        assert!(app.running);
        assert_eq!(app.input, "xq");
    }

    #[test]
    fn key_backspace_pops_input() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.handle_event(key_event(KeyCode::Char('a')));
        app.handle_event(key_event(KeyCode::Char('b')));
        app.handle_event(key_event(KeyCode::Backspace));
        assert_eq!(app.input, "a");
    }

    #[test]
    fn key_esc_clears_input() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.handle_event(key_event(KeyCode::Char('x')));
        app.handle_event(key_event(KeyCode::Esc));
        assert!(app.input.is_empty());
    }

    #[test]
    fn prev_conv_noop_when_empty() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.prev_conv();
        assert_eq!(app.active_idx, 0);
    }

    #[test]
    fn next_conv_noop_when_empty() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.next_conv();
        assert_eq!(app.active_idx, 0);
    }

    #[test]
    fn key_nav_works_with_conversations() {
        let db = test_db();
        let (tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        let (session_tx, _session_rx) = mpsc::channel(8);
        let (_ev_tx, ev_rx) = mpsc::channel(8);
        let session = ChatSession {
            peer_id: "alice".into(),
            sender: session_tx,
            receiver: ev_rx,
        };
        tx.try_send(session).unwrap();
        app.poll_network();
        assert_eq!(app.conversations.len(), 1);

        app.handle_event(key_event(KeyCode::Down));
        assert_eq!(app.active_idx, 0);
        app.handle_event(key_event(KeyCode::Char('k')));
        assert_eq!(app.active_idx, 0);
    }

    #[test]
    fn prev_next_conv_wraps() {
        let db = test_db();
        let (tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);

        for i in 0..3 {
            let s = mk_session(&format!("peer{i}"));
            tx.try_send(s).unwrap();
        }
        app.poll_network();
        assert_eq!(app.conversations.len(), 3);

        app.next_conv();
        assert_eq!(app.active_idx, 1);
        app.next_conv();
        assert_eq!(app.active_idx, 2);
        app.next_conv();
        assert_eq!(app.active_idx, 0);
        app.prev_conv();
        assert_eq!(app.active_idx, 2);
    }

    fn mk_session(peer: &str) -> ChatSession {
        let (tx, _rx) = mpsc::channel(8);
        let (ev_tx, ev_rx) = mpsc::channel(8);
        std::mem::forget(ev_tx); // keep receiver alive (no Disconnected)
        ChatSession {
            peer_id: peer.to_string(),
            sender: tx,
            receiver: ev_rx,
        }
    }

    #[test]
    fn send_empty_noop() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.send();
        assert!(app.running);
    }

    #[test]
    fn send_quit_exits() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.input = "/quit".into();
        app.send();
        assert!(!app.running);
        assert!(app.input.is_empty());
    }

    #[test]
    fn send_with_no_active_peer_shows_status() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        app.input = "hello".into();
        app.send();
        assert_eq!(app.status, "No conversation selected");
        assert!(app.input.is_empty());
    }

    #[test]
    fn send_with_online_peer_sends_message() {
        let db = test_db();
        let (tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        let (session_tx, mut session_rx) = mpsc::channel(8);
        let (_ev_tx, ev_rx) = mpsc::channel(8);
        let session = ChatSession {
            peer_id: "bob".into(),
            sender: session_tx,
            receiver: ev_rx,
        };
        tx.try_send(session).unwrap();
        app.poll_network();

        app.input = "hey".into();
        app.send();
        assert!(app.input.is_empty());

        // message should have been sent via session
        let sent = session_rx.try_recv().unwrap();
        assert_eq!(sent, Bytes::from("hey"));
    }

    #[test]
    fn poll_network_adds_contact_automatically() {
        let (tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(test_db(), rx);

        let session = mk_session("charlie");
        tx.try_send(session).unwrap();
        app.poll_network();

        let contacts = app.db.list_contacts().unwrap();
        assert!(contacts.iter().any(|c| c.peer_id == "charlie"));
    }

    #[test]
    fn active_peer_returns_none_when_empty() {
        let db = test_db();
        let (_tx, rx) = mpsc::channel(8);
        let app = TuiApp::new(db, rx);
        assert_eq!(app.active_peer(), None);
    }

    #[test]
    fn active_peer_returns_selected() {
        let db = test_db();
        let (tx, rx) = mpsc::channel(8);
        let mut app = TuiApp::new(db, rx);
        let session = mk_session("frank");
        tx.try_send(session).unwrap();
        app.poll_network();
        assert_eq!(app.active_peer(), Some("frank"));
    }
}
