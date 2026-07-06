use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid peer ID: {0}")]
    InvalidPeerId(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Crypto error: {0}")]
    Crypto(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io() {
        let err = CoreError::Io(std::io::Error::other("disk full"));
        assert_eq!(err.to_string(), "I/O error: disk full");
    }

    #[test]
    fn display_invalid_peer_id() {
        let err = CoreError::InvalidPeerId("bad".into());
        assert_eq!(err.to_string(), "Invalid peer ID: bad");
    }

    #[test]
    fn display_serialization() {
        let err = CoreError::Serialization("failed".into());
        assert_eq!(err.to_string(), "Serialization error: failed");
    }

    #[test]
    fn display_invalid_message() {
        let err = CoreError::InvalidMessage("too short".into());
        assert_eq!(err.to_string(), "Invalid message: too short");
    }

    #[test]
    fn display_crypto() {
        let err = CoreError::Crypto("verify failed".into());
        assert_eq!(err.to_string(), "Crypto error: verify failed");
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err: CoreError = io_err.into();
        assert!(matches!(err, CoreError::Io(_)));
    }
}
