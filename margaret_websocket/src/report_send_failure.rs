use crate::web_socket_error::WebSocketError;

pub(crate) fn report_send_failure(outcome: Result<(), WebSocketError>) {
    if let Err(error) = outcome {
        eprintln!("margaret_websocket: unable to deliver a websocket frame: {error}");
    }
}
