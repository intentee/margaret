use margaret_websocket_envelope::envelope_error::EnvelopeError;

#[derive(Debug)]
pub enum ResponseItem<Payload> {
    Payload(Payload),
    Rejected(EnvelopeError),
}
