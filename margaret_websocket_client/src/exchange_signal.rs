use margaret_websocket_envelope::request_id::RequestId;

pub(crate) enum ExchangeSignal {
    Cancelled(RequestId),
    Consumed(RequestId),
}
