#[must_use]
pub fn valid_raw_handshake(path: &str) -> Vec<u8> {
    format!(
        "GET {path} HTTP/1.1\r\nHost: test\r\nConnection: Upgrade, close\r\nUpgrade: websocket\r\nOrigin: https://example.test\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n"
    )
    .into_bytes()
}
