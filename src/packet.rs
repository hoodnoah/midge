use crate::error::MqttError;

/// MQTT communicates through the exchange of  MQTT control packets.
/// An MQTT packet is comprised of 3 parts, in the same order:
/// /// 1. The fixed header (all packets)
/// /// 2. The variable header (some packets)
/// /// 3. Payload (some packets)

const CONNECT_FLAGS: u8 = 0x00;
const CONNACK_FLAGS: u8 = 0x00;
const PUBACK_FLAGS: u8 = 0x00;
const PUBREC_FLAGS: u8 = 0x00;
const PUBREL_FLAGS: u8 = 0x02;
const PUBCOMP_FLAGS: u8 = 0x00;
const SUBSCRIBE_FLAGS: u8 = 0x02;
const SUBACK_FLAGS: u8 = 0x00;
const UNSUBSCRIBE_FLAGS: u8 = 0x02;
const UNSUBACK_FLAGS: u8 = 0x00;
const PINGREQ_FLAGS: u8 = 0x00;
const PINGRESP_FLAGS: u8 = 0x00;
const DISCONNECT_FLAGS: u8 = 0x00;
const AUTH_FLAGS: u8 = 0x00;

pub struct Packet {
    pub fixed_header: FixedHeader,
    pub variable_header: Option<VariableHeader>,
    pub payload: Option<Payload>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QOS {
    ATMOSTONCE = 0,
    ATLEASTONCE = 1,
    EXACTLYONCE = 2,
}

pub enum FixedHeader {
    Standard {
        packet_type: ControlPacketType,
    },
    Publish {
        packet_type: ControlPacketType,
        qos: QOS,
        dup: bool,
    },
}

impl FixedHeader {
    // constructor for a Fixed Header. Auto-applies
    // known constant flags; for PUBLISH, applies 0x00 since the flags cannot be known at compile time.
    // Returns an error when the packet type is not valid for a given fixed header, e.g.
    // RESERVED is always an invalid header, and PUBLISH is invalid for a standard header.
    // PUBLISH headers must be created with new_publish, since QOS and DUP flags are unknown at compile time.
    pub fn new(packet_type: ControlPacketType) -> Result<Self, MqttError> {
        match packet_type {
            ControlPacketType::RESERVED => return Err(MqttError::InvalidPacketType),
            ControlPacketType::PUBLISH => return Err(MqttError::InvalidPacketType),
            _ => Ok(FixedHeader::Standard { packet_type }),
        }
    }

    // constructor for a PUBLISH Fixed Header.
    // requires a QOS level (ATMOSTONCE, ATLEASTONCE, or EXACTLYONCE) and a DUP flag (true or false).
    // DUP refers to whether this is a re-sending of the message, true means it's a DUP, false means it's the first time.
    pub fn new_publish(qos: QOS, dup: bool) -> Result<Self, MqttError> {
        Ok(FixedHeader::Publish {
            packet_type: ControlPacketType::PUBLISH,
            qos: qos,
            dup: dup,
        })
    }

    // encodes the FixedHeader as a byte array.
    // the first 4 bits are the MQTT packet type, the next 4 bits are the flags.
    // for a PUBLISH header, the last 3 bits are the QOS level and the DUP flag, and the final bit is always 0.
    pub fn encode(&self) -> Result<[u8; 2], MqttError> {
        let mut header: [u8; 2] = [0x00, 0x00];

        match self {
            FixedHeader::Standard { packet_type } => {
                header[0] = (*packet_type as u8) << 4; // shift into first 4 bits
                // encode flags
                header[0] |= match *packet_type {
                    ControlPacketType::CONNECT => CONNECT_FLAGS,
                    ControlPacketType::CONNACK => CONNACK_FLAGS,
                    ControlPacketType::PUBACK => PUBACK_FLAGS,
                    ControlPacketType::PUBREC => PUBREC_FLAGS,
                    ControlPacketType::PUBREL => PUBREL_FLAGS,
                    ControlPacketType::PUBCOMP => PUBCOMP_FLAGS,
                    ControlPacketType::SUBSCRIBE => SUBSCRIBE_FLAGS,
                    ControlPacketType::SUBACK => SUBACK_FLAGS,
                    ControlPacketType::UNSUBSCRIBE => UNSUBSCRIBE_FLAGS,
                    ControlPacketType::UNSUBACK => UNSUBACK_FLAGS,
                    ControlPacketType::PINGREQ => PINGREQ_FLAGS,
                    ControlPacketType::PINGRESP => PINGRESP_FLAGS,
                    ControlPacketType::DISCONNECT => DISCONNECT_FLAGS,
                    ControlPacketType::AUTH => AUTH_FLAGS,
                    _ => return Err(MqttError::InvalidPacketType),
                };
            }
            FixedHeader::Publish {
                packet_type,
                qos,
                dup,
            } => {
                // encode packet type
                header[0] = (*packet_type as u8) << 4; // shift into first 4 bits

                // encode DUP flag (bit 3)
                if *dup {
                    header[0] |= 0x08; // set bit 3 to 1
                }

                // encode QOS flags
                header[0] |= (*qos as u8) << 1; // shift into the next 2 bits
            }
        }

        header[1] = 0x00; // placeholder for "remaining length" field

        Ok(header)
    }
}

pub struct VariableHeader {}
pub struct Payload {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ControlPacketType {
    RESERVED = 0,     // Reserved for future use
    CONNECT = 1,      // Client -> Server, connection request
    CONNACK = 2,      // Server -> Client, connection acknowledgement
    PUBLISH = 3,      // Client <-> Server, publish message (QoS 1)
    PUBACK = 4,       // Client <-> Server, publish acknowledgement (QoS 2 delivery part 1)
    PUBREC = 5,       // Client <-> Server, publish received (QoS 2 delivery part 2)
    PUBREL = 6,       // Client <-> Server, publish release (QoS 2 delivery part 3)
    PUBCOMP = 7,      // Client <-> Server, publish complete (QoS 2 delivery part 4)
    SUBSCRIBE = 8,    // Client -> Server, subscribe request
    SUBACK = 9,       // Server -> Client, subscribe acknowledgement
    UNSUBSCRIBE = 10, // Client -> Server, unsubscribe request
    UNSUBACK = 11,    // Server -> Client, unsubscribe acknowledgement
    PINGREQ = 12,     // Client <-> Server, ping request
    PINGRESP = 13,    // Client <-> Server, ping response
    DISCONNECT = 14,  // Client <-> Server, disconnect notification
    AUTH = 15,        // Client <-> Server, authentication exchange
}

#[cfg(test)]
mod test_fixed_header_encode {
    use super::*;

    #[test]
    fn test_encode_connect() {
        let header = FixedHeader::new(ControlPacketType::CONNECT).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b00010000, 0x00])
    }

    #[test]
    fn test_encode_connack() {
        let header = FixedHeader::new(ControlPacketType::CONNACK).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b00100000, 0x00])
    }

    #[test]
    fn test_encode_publish() {
        let headers = [
            FixedHeader::new_publish(QOS::ATMOSTONCE, false).unwrap(),
            FixedHeader::new_publish(QOS::ATLEASTONCE, false).unwrap(),
            FixedHeader::new_publish(QOS::EXACTLYONCE, false).unwrap(),
            FixedHeader::new_publish(QOS::ATMOSTONCE, true).unwrap(),
            FixedHeader::new_publish(QOS::ATLEASTONCE, true).unwrap(),
            FixedHeader::new_publish(QOS::EXACTLYONCE, true).unwrap(),
        ];
        let expected_headers: [[u8; 2]; 6] = [
            [0b00110000, 0x00],
            [0b00110010, 0x00],
            [0b00110100, 0x00],
            [0b00111000, 0x00],
            [0b00111010, 0x00],
            [0b00111100, 0x00],
        ];

        for (i, header) in headers.iter().enumerate() {
            let encoded = header.encode().unwrap();
            assert_eq!(encoded, expected_headers[i]);
        }
    }

    #[test]
    fn test_encode_puback() {
        let header = FixedHeader::new(ControlPacketType::PUBACK).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b01000000, 0x00])
    }

    #[test]
    fn test_encode_pubrec() {
        let header = FixedHeader::new(ControlPacketType::PUBREC).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b01010000, 0x00])
    }

    #[test]
    fn test_encode_pubrel() {
        let header = FixedHeader::new(ControlPacketType::PUBREL).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b01100010, 0x00])
    }

    #[test]
    fn test_encode_pubcomp() {
        let header = FixedHeader::new(ControlPacketType::PUBCOMP).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b01110000, 0x00])
    }

    #[test]
    fn test_encode_subscribe() {
        let header = FixedHeader::new(ControlPacketType::SUBSCRIBE).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b10000010, 0x00])
    }

    #[test]
    fn test_encode_suback() {
        let header = FixedHeader::new(ControlPacketType::SUBACK).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b10010000, 0x00])
    }

    #[test]
    fn test_encode_unsubscribe() {
        let header = FixedHeader::new(ControlPacketType::UNSUBSCRIBE).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b10100010, 0x00])
    }

    #[test]
    fn test_encode_unsuback() {
        let header = FixedHeader::new(ControlPacketType::UNSUBACK).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b10110000, 0x00])
    }

    #[test]
    fn test_encode_pingreq() {
        let header = FixedHeader::new(ControlPacketType::PINGREQ).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b11000000, 0x00])
    }

    #[test]
    fn test_encode_pingresp() {
        let header = FixedHeader::new(ControlPacketType::PINGRESP).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b11010000, 0x00])
    }

    #[test]
    fn test_encode_disconnect() {
        let header = FixedHeader::new(ControlPacketType::DISCONNECT).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b11100000, 0x00])
    }

    #[test]
    fn test_encode_auth() {
        let header = FixedHeader::new(ControlPacketType::AUTH).unwrap();
        let encoded = header.encode().unwrap();

        assert_eq!(encoded, [0b11110000, 0x00])
    }
}
