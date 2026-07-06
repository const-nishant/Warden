#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn new(port: u16) -> Self {
        Self {
            bind_addr: "0.0.0.0".to_string(),
            port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_default_bind() {
        let cfg = ServerConfig::new(2222);
        assert_eq!(cfg.bind_addr, "0.0.0.0");
        assert_eq!(cfg.port, 2222);
    }

    #[test]
    fn new_custom_port() {
        let cfg = ServerConfig::new(3333);
        assert_eq!(cfg.port, 3333);
    }

    #[test]
    fn debug_roundtrip() {
        let cfg = ServerConfig::new(4444);
        let s = format!("{cfg:?}");
        assert!(s.contains("4444"));
    }
}
