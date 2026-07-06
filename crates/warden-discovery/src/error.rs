use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiscoveryError {
    #[error("DHT error: {0}")]
    Dht(String),

    #[error("Bootstrap error: {0}")]
    Bootstrap(String),

    #[error("NAT traversal error: {0}")]
    NatTraversal(String),

    #[error("Keypair conversion error: {0}")]
    Keypair(String),

    #[error("Listen error: {0}")]
    Listen(String),

    #[error("Resolve error: {0}")]
    Resolve(String),

    #[error("Node shut down")]
    Shutdown,
}

impl From<std::convert::Infallible> for DiscoveryError {
    fn from(_: std::convert::Infallible) -> Self {
        DiscoveryError::Dht("infallible conversion".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_dht() {
        let err = DiscoveryError::Dht("timeout".into());
        assert_eq!(err.to_string(), "DHT error: timeout");
    }

    #[test]
    fn display_bootstrap() {
        let err = DiscoveryError::Bootstrap("no peers".into());
        assert_eq!(err.to_string(), "Bootstrap error: no peers");
    }

    #[test]
    fn display_nat_traversal() {
        let err = DiscoveryError::NatTraversal("stun failed".into());
        assert_eq!(err.to_string(), "NAT traversal error: stun failed");
    }

    #[test]
    fn display_keypair() {
        let err = DiscoveryError::Keypair("invalid".into());
        assert_eq!(err.to_string(), "Keypair conversion error: invalid");
    }

    #[test]
    fn display_listen() {
        let err = DiscoveryError::Listen("port in use".into());
        assert_eq!(err.to_string(), "Listen error: port in use");
    }

    #[test]
    fn display_resolve() {
        let err = DiscoveryError::Resolve("not found".into());
        assert_eq!(err.to_string(), "Resolve error: not found");
    }

    #[test]
    fn display_shutdown() {
        let err = DiscoveryError::Shutdown;
        assert_eq!(err.to_string(), "Node shut down");
    }

    #[test]
    fn from_infallible_can_be_called() {
        fn convert(x: std::convert::Infallible) -> DiscoveryError {
            x.into()
        }
        let inf: Result<(), std::convert::Infallible> = Ok(());
        let _ = inf.map_err(convert);
        // If this compiles, the From impl exists and works
    }
}
