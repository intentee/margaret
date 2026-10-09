use std::future::Future;

use futures_util::StreamExt;
use futures_util::stream::FuturesUnordered;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

use crate::shuts_down_gracefully::ShutsDownGracefully;
use crate::web_socket_driver::WebSocketDriver;

pub(crate) async fn drive_connection<TConnection, TError>(
    connection: TConnection,
    mut driver_receiver: Receiver<WebSocketDriver>,
    cancellation_token: CancellationToken,
) -> Result<(), TError>
where
    TConnection: Future<Output = Result<(), TError>> + ShutsDownGracefully,
{
    tokio::pin!(connection);
    let mut drivers = FuturesUnordered::new();
    let mut shutting_down = false;

    let outcome = loop {
        tokio::select! {
            biased;
            outcome = &mut connection => break outcome,
            () = cancellation_token.cancelled(), if !shutting_down => {
                connection.as_mut().shut_down_gracefully();
                shutting_down = true;
            }
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
    use std::future::Future;
    use std::io;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::task::Context;
    use std::task::Poll;

    use tokio::sync::oneshot;
    use tokio_util::sync::CancellationToken;

    use super::drive_connection;
    use crate::shuts_down_gracefully::ShutsDownGracefully;
    use crate::web_socket_driver_channel::WebSocketDriverChannel;

    struct ScriptedConnection {
        completion: Pin<Box<dyn Future<Output = Result<(), io::Error>> + Send>>,
        shutdown: CancellationToken,
    }

    impl ScriptedConnection {
        fn completing(
            completion: impl Future<Output = Result<(), io::Error>> + Send + 'static,
        ) -> Self {
            Self {
                completion: Box::pin(completion),
                shutdown: CancellationToken::new(),
            }
        }
    }

    impl Future for ScriptedConnection {
        type Output = Result<(), io::Error>;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            self.completion.as_mut().poll(context)
        }
    }

    impl ShutsDownGracefully for ScriptedConnection {
        fn shut_down_gracefully(self: Pin<&mut Self>) {
            self.shutdown.cancel();
        }
    }

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
            ScriptedConnection::completing(async move {
                connection_finished
                    .await
                    .expect("the test requests connection completion");
                Ok(())
            }),
            receiver,
            CancellationToken::new(),
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

        drive_connection(
            ScriptedConnection::completing(async { Ok(()) }),
            receiver,
            CancellationToken::new(),
        )
        .await
        .expect("the connection succeeds");

        assert!(completed.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn shuts_the_connection_down_gracefully_on_cancellation() {
        let WebSocketDriverChannel { receiver, .. } = WebSocketDriverChannel::new();
        let shutdown = CancellationToken::new();
        let awaited_shutdown = shutdown.clone();
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel();
        drive_connection(
            ScriptedConnection {
                completion: Box::pin(async move {
                    awaited_shutdown.cancelled().await;

                    Ok(())
                }),
                shutdown,
            },
            receiver,
            cancellation_token,
        )
        .await
        .expect("the connection finishes once it shuts down");
    }
}
