use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Key generation error: {0}")]
    KeyGeneration(String),

    #[error("Signature error: {0}")]
    Signature(String),

    #[error("Encoding error: {0}")]
    Encoding(String),
}

impl From<ed25519_dalek::ed25519::Error> for IdentityError {
    fn from(e: ed25519_dalek::ed25519::Error) -> Self {
        IdentityError::Signature(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io() {
        let err = IdentityError::Io(std::io::Error::other("eof"));
        assert_eq!(err.to_string(), "I/O error: eof");
    }

    #[test]
    fn display_key_generation() {
        let err = IdentityError::KeyGeneration("bad seed".into());
        assert_eq!(err.to_string(), "Key generation error: bad seed");
    }

    #[test]
    fn display_signature() {
        let err = IdentityError::Signature("invalid".into());
        assert_eq!(err.to_string(), "Signature error: invalid");
    }

    #[test]
    fn display_encoding() {
        let err = IdentityError::Encoding("base58".into());
        assert_eq!(err.to_string(), "Encoding error: base58");
    }

    #[test]
    fn from_ed25519_error() {
        let e = ed25519_dalek::ed25519::Error::from(
            Box::new(std::io::Error::other("test"))
                as Box<dyn std::error::Error + Send + Sync>,
        );
        let err: IdentityError = e.into();
        assert!(matches!(err, IdentityError::Signature(_)));
    }
}
