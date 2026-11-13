//! Discord Voice UDP Transport: IP Discovery & Custom RTP Framing

/// 74-byte Discord UDP IP Discovery Packet
///
/// Sent by the client to discover its public external IP address and UDP port
/// through NAT before selecting the voice protocol.
#[derive(Debug, Clone)]
pub struct IpDiscoveryPacket {
    pub packet_type: u16, // 0x0001 (Request) or 0x0002 (Response)
    pub length: u16,      // 70 bytes
    pub ssrc: u32,
    pub address: String,  // Null-terminated string (up to 64 bytes)
    pub port: u16,
}

impl IpDiscoveryPacket {
    /// Construct a new IP discovery request packet for the given SSRC
    pub fn new_request(ssrc: u32) -> [u8; 74] {
        let mut buffer = [0u8; 74];
        // Packet Type: 0x0001 (Big-Endian)
        buffer[0] = 0x00;
        buffer[1] = 0x01;
        // Length: 70 bytes (Big-Endian)
        buffer[2] = 0x00;
        buffer[3] = 0x46;
        // SSRC (Big-Endian)
        buffer[4..8].copy_from_slice(&ssrc.to_be_bytes());
        buffer
    }

    /// Parse an IP discovery response packet received from Discord voice server
    pub fn parse_response(data: &[u8]) -> Result<(String, u16), &'static str> {
        if data.len() < 74 {
            return Err("Packet too short for IP discovery response");
        }
        let packet_type = u16::from_be_bytes([data[0], data[1]]);
        if packet_type != 0x0002 {
            return Err("Invalid response packet type");
        }
        // Extract IP string up to null terminator (bytes 8..72)
        let ip_bytes = &data[8..72];
        let null_pos = ip_bytes.iter().position(|&b| b == 0).unwrap_or(ip_bytes.len());
        let ip_str = std::str::from_utf8(&ip_bytes[..null_pos]).map_err(|_| "Invalid UTF-8 in IP")?;
        // Extract port: 16-bit little-endian at bytes 72..74
        let port = u16::from_le_bytes([data[72], data[73]]);
        Ok((ip_str.to_string(), port))
    }
}

/// Discord Custom RTP Header (12 bytes)
#[derive(Debug, Clone, Copy)]
pub struct RtpHeader {
    pub payload_type: u8, // 120 (0x78) for Opus
    pub sequence: u16,
    pub timestamp: u32,
    pub ssrc: u32,
}

impl RtpHeader {
    pub const OPUS_PAYLOAD_TYPE: u8 = 120;

    pub fn new(sequence: u16, timestamp: u32, ssrc: u32) -> Self {
        Self {
            payload_type: Self::OPUS_PAYLOAD_TYPE,
            sequence,
            timestamp,
            ssrc,
        }
    }

    pub fn write_to(&self, buffer: &mut [u8]) {
        assert!(buffer.len() >= 12, "Buffer too short for RTP header");
        buffer[0] = 0x80; // Version 2, no padding, no extension, no CSRC
        buffer[1] = self.payload_type & 0x7F;
        buffer[2..4].copy_from_slice(&self.sequence.to_be_bytes());
        buffer[4..8].copy_from_slice(&self.timestamp.to_be_bytes());
        buffer[8..12].copy_from_slice(&self.ssrc.to_be_bytes());
    }
}
