use rustls::Error;

#[derive(Debug)]
pub enum HandshakeError {
    Tls(Error),
    ExceededMaxRounds,
}

impl From<Error> for HandshakeError {
    fn from(error: Error) -> Self {
        HandshakeError::Tls(error)
    }
}
