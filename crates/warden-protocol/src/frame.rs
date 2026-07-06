use bytes::Bytes;
use prost::Message;
use warden_core::types::proto::{ChatFrame as ProtoChatFrame, FrameType};

use crate::ProtocolError;

#[derive(Debug, Clone)]
pub struct MessageFrame {
    pub frame_type: FrameType,
    pub message_id: String,
    pub sender_peer_id: String,
    pub timestamp_unix_ms: i64,
    pub ciphertext: Vec<u8>,
    pub signature: Vec<u8>,
}

impl MessageFrame {
    pub fn encode(&self) -> Result<Bytes, ProtocolError> {
        let proto = ProtoChatFrame {
            message_id: self.message_id.clone(),
            sender_peer_id: self.sender_peer_id.clone(),
            timestamp_unix_ms: self.timestamp_unix_ms,
            ciphertext: self.ciphertext.clone(),
            signature: self.signature.clone(),
            frame_type: self.frame_type.into(),
        };

        let mut buf = Vec::new();
        proto.encode(&mut buf).map_err(|e: prost::EncodeError| {
            ProtocolError::Framing(e.to_string())
        })?;

        Ok(Bytes::from(buf))
    }

    pub fn decode(data: &[u8]) -> Result<Self, ProtocolError> {
        let proto = ProtoChatFrame::decode(data).map_err(|e: prost::DecodeError| {
            ProtocolError::Framing(e.to_string())
        })?;

        Ok(Self {
            frame_type: FrameType::try_from(proto.frame_type)
                .unwrap_or(FrameType::Unspecified),
            message_id: proto.message_id,
            sender_peer_id: proto.sender_peer_id,
            timestamp_unix_ms: proto.timestamp_unix_ms,
            ciphertext: proto.ciphertext,
            signature: proto.signature,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warden_core::types::proto::FrameType;

    #[test]
    fn encode_decode_roundtrip() {
        let frame = MessageFrame {
            frame_type: FrameType::Text,
            message_id: "mid1".into(),
            sender_peer_id: "alice".into(),
            timestamp_unix_ms: 1700000000000,
            ciphertext: b"hello".to_vec(),
            signature: vec![4, 5, 6],
        };
        let encoded = frame.encode().unwrap();
        let decoded = MessageFrame::decode(&encoded).unwrap();
        assert_eq!(decoded.message_id, "mid1");
        assert_eq!(decoded.sender_peer_id, "alice");
        assert_eq!(decoded.ciphertext, b"hello");
        assert_eq!(decoded.signature, vec![4, 5, 6]);
        assert_eq!(decoded.frame_type, FrameType::Text);
    }

    #[test]
    fn decode_invalid_data() {
        let err = MessageFrame::decode(b"garbage").unwrap_err();
        assert!(matches!(err, ProtocolError::Framing(_)));
    }

    #[test]
    fn encode_ack_frame() {
        let frame = MessageFrame {
            frame_type: FrameType::Ack,
            message_id: "ack1".into(),
            sender_peer_id: "bob".into(),
            timestamp_unix_ms: 1000,
            ciphertext: vec![],
            signature: vec![],
        };
        let encoded = frame.encode().unwrap();
        let decoded = MessageFrame::decode(&encoded).unwrap();
        assert_eq!(decoded.frame_type, FrameType::Ack);
    }

    #[test]
    fn encode_empty_fields() {
        let frame = MessageFrame {
            frame_type: FrameType::Unspecified,
            message_id: String::new(),
            sender_peer_id: String::new(),
            timestamp_unix_ms: 0,
            ciphertext: vec![],
            signature: vec![],
        };
        let encoded = frame.encode().unwrap();
        let decoded = MessageFrame::decode(&encoded).unwrap();
        assert_eq!(decoded.message_id, "");
        assert_eq!(decoded.timestamp_unix_ms, 0);
    }
}
