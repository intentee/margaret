use std::future::Future;

use futures_util::StreamExt;
use futures_util::stream::FuturesUnordered;
use tokio::sync::mpsc::Receiver;

use crate::web_socket_driver::WebSocketDriver;

pub(crate) async fn drive_connection<Connection, Error>(
    connection: Connection,
    mut driver_receiver: Receiver<WebSocketDriver>,
) -> Result<(), Error>
where
    Connection: Future<Output = Result<(), Error>>,
{
    tokio::pin!(connection);
    let mut drivers = FuturesUnordered::new();

    let outcome = loop {
        tokio::select! {
            biased;
            outcome = &mut connection => break outcome,
            Some(driver) = driver_receiver.recv() => drivers.push(driver),
            Some(()) = drivers.next(), if !drivers.is_empty() => {}
        }
    };
    driver_receiver.close();

    while let Ok(driver) = driver_receiver.try_recv() {
        drivers.push(driver);
    }

    while drivers.next().await.is_some() {}

    outcome
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use tokio::sync::oneshot;

    use super::drive_connection;
    use crate::web_socket_driver_channel::WebSocketDriverChannel;

    #[tokio::test]
    async fn polls_a_received_driver_while_the_connection_is_open() {
        let WebSocketDriverChannel { receiver, sender } = WebSocketDriverChannel::new();
        let (driver_done, observed_driver) = oneshot::channel();
        assert!(
            sender
                .send(Box::pin(async move {
                    driver_done
                        .send(())
                        .expect("the test observes the driver completion");
                }))
                .await
                .is_ok()
        );
        let (finish_connection, connection_finished) = oneshot::channel();
        let driving = tokio::spawn(drive_connection(
            async move {
                connection_finished
                    .await
                    .expect("the test requests connection completion");
                Ok::<(), io::Error>(())
            },
            receiver,
        ));

        observed_driver
            .await
            .expect("the received driver is polled");
        finish_connection
            .send(())
            .expect("the connection task remains active");

        driving
            .await
            .expect("the driving task completes")
            .expect("the connection succeeds");
    }

    #[tokio::test]
    async fn drains_a_queued_driver_after_the_connection_finishes() {
        let WebSocketDriverChannel { receiver, sender } = WebSocketDriverChannel::new();
        let completed = Arc::new(AtomicBool::new(false));
        let completed_by_driver = Arc::clone(&completed);
        assert!(
            sender
                .send(Box::pin(async move {
                    completed_by_driver.store(true, Ordering::SeqCst);
                }))
                .await
                .is_ok()
        );

        drive_connection(async { Ok::<(), io::Error>(()) }, receiver)
            .await
            .expect("the connection succeeds");

        assert!(completed.load(Ordering::SeqCst));
    }
}
