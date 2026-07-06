use std::collections::HashMap;
use std::path::PathBuf;

use russh_keys::key::PublicKey;
use russh_keys::PublicKeyBase64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostKeyEntry {
    pub host: String,
    pub key_type: String,
    pub key_blob: Vec<u8>,
}

#[derive(Debug)]
pub struct KnownHosts {
    path: PathBuf,
    entries: HashMap<String, HostKeyEntry>,
}

impl KnownHosts {
    pub fn new(path: PathBuf) -> Self {
        let entries = if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            HashMap::new()
        };
        Self { path, entries }
    }

    pub fn lookup(&self, host: &str) -> Option<&HostKeyEntry> {
        self.entries.get(host)
    }

    pub fn insert(&mut self, host: &str, key: &PublicKey) {
        let entry = HostKeyEntry {
            host: host.to_string(),
            key_type: key_name(key),
            key_blob: key.public_key_base64().into_bytes(),
        };
        self.entries.insert(host.to_string(), entry);
        self.save();
    }

    pub fn verify(&mut self, host: &str, key: &PublicKey) -> Result<bool, String> {
        match self.lookup(host) {
            None => {
                self.insert(host, key);
                Ok(true)
            }
            Some(expected) => {
                let got = key.public_key_base64().into_bytes();
                Ok(expected.key_blob == got)
            }
        }
    }

    fn save(&self) {
        if let Some(dir) = self.path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string(&self.entries) {
            let _ = std::fs::write(&self.path, json);
        }
    }
}

fn key_name(key: &PublicKey) -> String {
    match key {
        PublicKey::Ed25519(_) => "ssh-ed25519".into(),
        PublicKey::RSA { .. } => "ssh-rsa".into(),
        _ => "unknown".into(),
    }
}
