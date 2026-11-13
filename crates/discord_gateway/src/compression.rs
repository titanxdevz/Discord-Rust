use std::io::Read;
use flate2::Decompress;

pub const ZLIB_SUFFIX: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];

/// Streaming Zlib decompressor for Discord Gateway (compress=zlib-stream)
pub struct ZlibStreamDecompressor {
    buffer: Vec<u8>,
    decompressor: Decompress,
}

impl ZlibStreamDecompressor {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(32 * 1024),
            decompressor: Decompress::new(true),
        }
    }

    /// Feed chunked bytes from WebSocket; if packet contains ZLIB_SUFFIX, decompress and return JSON string
    pub fn feed(&mut self, chunk: &[u8]) -> Result<Option<String>, std::io::Error> {
        self.buffer.extend_from_slice(chunk);

        if self.buffer.len() >= 4 && self.buffer.ends_with(&ZLIB_SUFFIX) {
            let mut decompressed = Vec::new();
            let mut decompress = flate2::read::ZlibDecoder::new(&self.buffer[..]);
            decompress.read_to_end(&mut decompressed)?;
            self.buffer.clear();

            let json_str = String::from_utf8(decompressed)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(Some(json_str))
        } else {
            Ok(None)
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.decompressor.reset(true);
    }
}

impl Default for ZlibStreamDecompressor {
    fn default() -> Self {
        Self::new()
    }
}
