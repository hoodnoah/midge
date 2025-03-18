#[derive(Debug, PartialEq)]
pub enum MqttError {
    InvalidPacketType,
    InvalidQOSLevel,
    InvalidRetries,
}
