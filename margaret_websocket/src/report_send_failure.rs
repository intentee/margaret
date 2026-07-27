use crate::web_socket_error::WebSocketError;

pub(crate) fn report_send_failure(outcome: Result<(), WebSocketError>) {
    if let Err(error) = outcome {
        eprintln!("margaret_websocket: unable to deliver a websocket frame: {error}");
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;
    use tokio_tungstenite::tungstenite::Message;

    use super::report_send_failure;
    use crate::web_socket_error::WebSocketError;

    #[tokio::test]
    async fn reports_a_frame_that_cannot_be_delivered() {
        let (sender, receiver) = mpsc::channel(1);
        drop(receiver);
        let source = sender
            .send(Message::text("frame"))
            .await
            .expect_err("the closed channel rejects the frame");

        report_send_failure(Err(WebSocketError::Send { source }));
    }
}
