use margaret_websocket_envelope::envelope_error::EnvelopeError;

pub enum ResponseItem<Payload> {
    Payload(Payload),
    Rejected(EnvelopeError),
}
