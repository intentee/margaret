use rustls::Error;
use rustls::client::danger::HandshakeSignatureValid;

/// # Errors
///
/// Returns `Error::General`.
pub fn reject_tls12_signature() -> Result<HandshakeSignatureValid, Error> {
    Err(Error::General("TLS 1.2 is not supported".to_string()))
}
