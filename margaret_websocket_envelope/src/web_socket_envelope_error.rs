use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebSocketEnvelopeError {
    #[error(
        "the frame grants {credit} response frames of credit, but an exchange may not be granted more than {maximum}"
    )]
    CreditGrantOutOfRange { credit: usize, maximum: usize },
}
