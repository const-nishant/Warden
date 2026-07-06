use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SSH error: {0}")]
    Ssh(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Subsystem error: {0}")]
    Subsystem(String),
}

impl From<russh::Error> for TransportError {
    fn from(e: russh::Error) -> Self {
        TransportError::Ssh(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io() {
        let err = TransportError::Io(std::io::Error::other("reset"));
        assert_eq!(err.to_string(), "I/O error: reset");
    }

    #[test]
    fn display_ssh() {
        let err = TransportError::Ssh("key exchange failed".into());
        assert_eq!(err.to_string(), "SSH error: key exchange failed");
    }

    #[test]
    fn display_auth() {
        let err = TransportError::Auth("bad key".into());
        assert_eq!(err.to_string(), "Authentication error: bad key");
    }

    #[test]
    fn display_connection() {
        let err = TransportError::Connection("refused".into());
        assert_eq!(err.to_string(), "Connection error: refused");
    }

    #[test]
    fn display_subsystem() {
        let err = TransportError::Subsystem("not supported".into());
        assert_eq!(err.to_string(), "Subsystem error: not supported");
    }
}
