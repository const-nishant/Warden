use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};
use warden_core::PeerId;

/// Derive a PeerID = bs58(sha256(pubkey_bytes)).
pub fn derive_peer_id(public_key: &[u8]) -> PeerId {
    let hash = Sha256::digest(public_key);
    let encoded = bs58::encode(hash).into_string();
    PeerId::new(encoded)
}

/// Create a PeerId from an Ed25519 verifying key.
pub fn peer_id_from_verifying_key(vk: &VerifyingKey) -> PeerId {
    derive_peer_id(&vk.to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_is_deterministic() {
        let pubkey = [0u8; 32];
        let id1 = derive_peer_id(&pubkey);
        let id2 = derive_peer_id(&pubkey);
        assert_eq!(id1, id2);
    }

    #[test]
    fn derive_differs_for_diff_keys() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];
        assert_ne!(derive_peer_id(&key1), derive_peer_id(&key2));
    }

    #[test]
    fn derive_returns_nonempty() {
        let pubkey = [0xab; 32];
        let id = derive_peer_id(&pubkey);
        assert!(!id.as_str().is_empty());
    }

    #[test]
    fn derive_is_bs58_encoded() {
        let pubkey = [0u8; 32];
        let id = derive_peer_id(&pubkey);
        // bs58 alphabet: no 0,O,I,l
        let s = id.as_str();
        assert!(s.len() >= 40);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn from_verifying_key_matches_derive() {
        let sk = ed25519_dalek::SigningKey::from_bytes(&[7u8; 32]);
        let vk = sk.verifying_key();
        let from_vk = peer_id_from_verifying_key(&vk);
        let from_raw = derive_peer_id(&vk.to_bytes());
        assert_eq!(from_vk, from_raw);
    }
}
