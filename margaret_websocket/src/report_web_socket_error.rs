use crate::web_socket_error::WebSocketError;

pub(crate) fn report_web_socket_error(outcome: Result<(), WebSocketError>) {
    if let Err(error) = outcome {
        eprintln!("margaret_websocket: {error}");
    }
}
