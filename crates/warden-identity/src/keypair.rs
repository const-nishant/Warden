use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use std::path::Path;

use crate::peer_id;
use crate::IdentityError;
use warden_core::PeerId;

#[derive(Debug)]
pub struct IdentityKeypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    peer_id: PeerId,
}

impl IdentityKeypair {
    pub fn generate() -> Self {
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        let peer_id = peer_id::peer_id_from_verifying_key(&verifying_key);

        Self {
            signing_key,
            verifying_key,
            peer_id,
        }
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, IdentityError> {
        let arr: [u8; 64] = bytes.try_into().map_err(|_| {
            IdentityError::KeyGeneration("invalid key bytes, expected 64".into())
        })?;
        let signing_key = SigningKey::from_keypair_bytes(&arr)?;
        let verifying_key = signing_key.verifying_key();
        let peer_id = peer_id::peer_id_from_verifying_key(&verifying_key);

        Ok(Self {
            signing_key,
            verifying_key,
            peer_id,
        })
    }

    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        self.signing_key.to_keypair_bytes()
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), IdentityError> {
        let bytes = self.to_bytes();
        std::fs::write(path.as_ref(), bytes)?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, IdentityError> {
        let bytes = std::fs::read(path.as_ref())?;
        Self::from_slice(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_get_peer_id() {
        let kp = IdentityKeypair::generate();
        let pid = kp.peer_id();
        assert!(!pid.as_str().is_empty());
    }

    #[test]
    fn to_from_bytes_roundtrip() {
        let kp = IdentityKeypair::generate();
        let bytes = kp.to_bytes();
        let restored = IdentityKeypair::from_slice(&bytes).unwrap();
        assert_eq!(kp.peer_id(), restored.peer_id());
    }

    #[test]
    fn from_slice_rejects_short_input() {
        let err = IdentityKeypair::from_slice(&[0u8; 32]).unwrap_err();
        assert!(matches!(err, IdentityError::KeyGeneration(_)));
    }

    #[test]
    fn verifying_key_stable() {
        let kp = IdentityKeypair::generate();
        let vk1 = kp.verifying_key().to_bytes();
        let vk2 = kp.verifying_key().to_bytes();
        assert_eq!(vk1, vk2);
    }

    #[test]
    fn save_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!("warden_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("id_test");
        let kp = IdentityKeypair::generate();
        kp.save(&path).unwrap();
        let loaded = IdentityKeypair::load(&path).unwrap();
        assert_eq!(kp.peer_id(), loaded.peer_id());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
