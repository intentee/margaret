use std::future::Future;

use futures_util::StreamExt;
use futures_util::stream::FuturesUnordered;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::connection_driver::ConnectionDriver;

pub(crate) async fn drive_connection<Connection, Error>(
    connection: Connection,
    mut driver_receiver: UnboundedReceiver<ConnectionDriver>,
) -> Result<(), Error>
where
    Connection: Future<Output = Result<(), Error>>,
{
    let mut connection = Box::pin(connection);
    let mut drivers = FuturesUnordered::new();

    let outcome = loop {
        tokio::select! {
            biased;
            outcome = &mut connection => break outcome,
            Some(driver) = driver_receiver.recv() => drivers.push(driver),
            Some(()) = drivers.next(), if !drivers.is_empty() => {}
        }
    };

    drop(connection);
    driver_receiver.close();

    while let Ok(driver) = driver_receiver.try_recv() {
        drivers.push(driver);
    }

    while drivers.next().await.is_some() {}

    outcome
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::io;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::task::Context;
    use std::task::Poll;

    use tokio::sync::oneshot;

    use super::drive_connection;
    use crate::connection_driver::ConnectionDriver;
    use crate::connection_driver_channel::ConnectionDriverChannel;
    use crate::connection_driver_sender::ConnectionDriverSender;

    struct HandingOffConnection {
        driver_sender: ConnectionDriverSender,
        handed_off: Arc<AtomicBool>,
    }

    impl Future for HandingOffConnection {
        type Output = Result<(), io::Error>;

        fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(Ok(()))
        }
    }

    impl Drop for HandingOffConnection {
        fn drop(&mut self) {
            let handed_off = Arc::clone(&self.handed_off);
            let driver: ConnectionDriver = Box::pin(async move {
                handed_off.store(true, Ordering::SeqCst);
            });

            assert!(
                self.driver_sender.send(driver).is_ok(),
                "the connection is dropped while its driver channel is still open"
            );
        }
    }

    #[tokio::test]
    async fn polls_a_received_driver_while_the_connection_is_open() {
        let ConnectionDriverChannel { receiver, sender } = ConnectionDriverChannel::new();
        let (driver_done, observed_driver) = oneshot::channel();
        assert!(
            sender
                .send(Box::pin(async move {
                    driver_done
                        .send(())
                        .expect("the test observes the driver completion");
                }))
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
        let ConnectionDriverChannel { receiver, sender } = ConnectionDriverChannel::new();
        let completed = Arc::new(AtomicBool::new(false));
        let completed_by_driver = Arc::clone(&completed);
        assert!(
            sender
                .send(Box::pin(async move {
                    completed_by_driver.store(true, Ordering::SeqCst);
                }))
                .is_ok()
        );

        drive_connection(async { Ok::<(), io::Error>(()) }, receiver)
            .await
            .expect("the connection succeeds");

        assert!(completed.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn drops_the_connection_before_draining_so_a_late_handoff_still_runs() {
        let ConnectionDriverChannel { receiver, sender } = ConnectionDriverChannel::new();
        let handed_off = Arc::new(AtomicBool::new(false));

        drive_connection(
            HandingOffConnection {
                driver_sender: sender,
                handed_off: Arc::clone(&handed_off),
            },
            receiver,
        )
        .await
        .expect("the connection succeeds");

        assert!(handed_off.load(Ordering::SeqCst));
    }
}
