use ed25519_dalek::{Signature as DalekSignature, Signer, Verifier};

use crate::{IdentityError, IdentityKeypair};

pub struct Signature(Vec<u8>);

impl Signature {
    pub fn to_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, IdentityError> {
        Ok(Self(bytes.to_vec()))
    }
}

pub trait Signing {
    fn sign(&self, data: &[u8]) -> Result<Signature, IdentityError>;
    fn verify(&self, data: &[u8], signature: &Signature) -> Result<bool, IdentityError>;
}

impl Signing for IdentityKeypair {
    fn sign(&self, data: &[u8]) -> Result<Signature, IdentityError> {
        let sig: DalekSignature = self.signing_key().sign(data);
        Ok(Signature(sig.to_bytes().to_vec()))
    }

    fn verify(&self, data: &[u8], signature: &Signature) -> Result<bool, IdentityError> {
        let sig = DalekSignature::from_slice(signature.to_bytes()).map_err(|e| {
            IdentityError::Signature(format!("invalid signature bytes: {e}"))
        })?;
        Ok(self.verifying_key().verify(data, &sig).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IdentityKeypair;

    #[test]
    fn sign_and_verify() {
        let kp = IdentityKeypair::generate();
        let data = b"hello warden";
        let sig = kp.sign(data).unwrap();
        assert!(kp.verify(data, &sig).unwrap());
    }

    #[test]
    fn verify_rejects_tampered_data() {
        let kp = IdentityKeypair::generate();
        let data = b"original message";
        let sig = kp.sign(data).unwrap();
        assert!(!kp.verify(b"tampered message", &sig).unwrap());
    }

    #[test]
    fn verify_rejects_wrong_key() {
        let kp1 = IdentityKeypair::generate();
        let kp2 = IdentityKeypair::generate();
        let data = b"test";
        let sig = kp1.sign(data).unwrap();
        assert!(!kp2.verify(data, &sig).unwrap());
    }

    #[test]
    fn signature_to_from_bytes() {
        let kp = IdentityKeypair::generate();
        let sig = kp.sign(b"bytes roundtrip").unwrap();
        let bytes = sig.to_bytes().to_vec();
        let restored = Signature::from_bytes(&bytes).unwrap();
        assert_eq!(sig.to_bytes(), restored.to_bytes());
    }

    #[test]
    fn sign_deterministic_ed25519() {
        let kp = IdentityKeypair::generate();
        let data = b"deterministic";
        let sig1 = kp.sign(data).unwrap();
        let sig2 = kp.sign(data).unwrap();
        assert_eq!(sig1.to_bytes(), sig2.to_bytes());
    }
}
