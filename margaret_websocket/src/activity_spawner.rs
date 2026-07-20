use std::future::Future;

use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

pub struct ActivitySpawner<Internal> {
    cancellation_token: CancellationToken,
    internal_sender: Sender<Internal>,
}

impl<Internal> ActivitySpawner<Internal>
where
    Internal: Send + 'static,
{
    #[must_use]
    pub fn new(cancellation_token: CancellationToken, internal_sender: Sender<Internal>) -> Self {
        Self {
            cancellation_token,
            internal_sender,
        }
    }

    #[must_use]
    pub fn internal_sender(&self) -> Sender<Internal> {
        self.internal_sender.clone()
    }

    pub fn spawn(&self, activity: impl Future<Output = ()> + Send + 'static) {
        let cancellation_token = self.cancellation_token.child_token();

        tokio::spawn(async move {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => {}
                () = activity => {}
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::channel;
    use tokio_util::sync::CancellationToken;

    use super::ActivitySpawner;

    #[tokio::test]
    async fn spawn_runs_the_activity_to_completion() {
        let (internal_sender, mut internal_receiver) = channel(1);
        let spawner = ActivitySpawner::new(CancellationToken::new(), internal_sender);
        let deliver = spawner.internal_sender();

        spawner.spawn(async move {
            deliver
                .send(7u8)
                .await
                .expect("the delivered event reaches the loop");
        });

        assert_eq!(internal_receiver.recv().await, Some(7));
    }

    #[tokio::test]
    async fn spawn_cancels_a_running_activity_when_the_connection_is_cancelled() {
        let (internal_sender, mut internal_receiver) = channel::<u8>(1);
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel();

        let spawner = ActivitySpawner::new(cancellation_token, internal_sender);

        spawner.spawn(std::future::pending());

        tokio::task::yield_now().await;

        drop(spawner);

        assert_eq!(internal_receiver.recv().await, None);
    }
}
