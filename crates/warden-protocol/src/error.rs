use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Framing error: {0}")]
    Framing(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Delivery error: {0}")]
    Delivery(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_framing() {
        let err = ProtocolError::Framing("invalid length".into());
        assert_eq!(err.to_string(), "Framing error: invalid length");
    }

    #[test]
    fn display_serialization() {
        let err = ProtocolError::Serialization("protobuf".into());
        assert_eq!(err.to_string(), "Serialization error: protobuf");
    }

    #[test]
    fn display_session() {
        let err = ProtocolError::Session("closed".into());
        assert_eq!(err.to_string(), "Session error: closed");
    }

    #[test]
    fn display_delivery() {
        let err = ProtocolError::Delivery("offline".into());
        assert_eq!(err.to_string(), "Delivery error: offline");
    }
}
