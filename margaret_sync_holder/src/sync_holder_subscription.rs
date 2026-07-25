use tokio::sync::watch::Receiver;
use tokio_util::sync::CancellationToken;

use crate::sync_holder_presence::SyncHolderPresence;

pub struct SyncHolderSubscription<TItem> {
    receiver: Receiver<Option<TItem>>,
}

impl<TItem: Clone + Send + Sync + 'static> SyncHolderSubscription<TItem> {
    #[must_use]
    pub fn new(receiver: Receiver<Option<TItem>>) -> Self {
        Self { receiver }
    }

    pub async fn changed(&mut self) {
        if self.receiver.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }

    pub fn read_current(&mut self) -> Option<TItem> {
        self.receiver.borrow_and_update().clone()
    }

    pub async fn wait_until_present(
        &mut self,
        cancellation_token: &CancellationToken,
    ) -> SyncHolderPresence {
        loop {
            if self.read_current().is_some() {
                return SyncHolderPresence::Present;
            }

            tokio::select! {
                () = cancellation_token.cancelled() => return SyncHolderPresence::Cancelled,
                () = self.changed() => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::pin;
    use std::task::Context;
    use std::task::Waker;

    use tokio_util::sync::CancellationToken;

    use crate::sync_holder::SyncHolder;
    use crate::sync_holder_presence::SyncHolderPresence;

    #[tokio::test]
    async fn read_current_returns_set_value() {
        let holder: SyncHolder<i32> = SyncHolder::default();

        holder.set(Some(10));

        let mut subscription = holder.subscribe();

        assert_eq!(subscription.read_current(), Some(10));
    }

    #[tokio::test]
    async fn changed_resolves_after_set_following_read_current() {
        let holder: SyncHolder<i32> = SyncHolder::default();
        let mut subscription = holder.subscribe();

        subscription.read_current();
        holder.set(Some(5));
        subscription.changed().await;

        assert_eq!(subscription.read_current(), Some(5));
    }

    #[tokio::test]
    async fn wait_until_present_returns_present_when_already_set() {
        let holder: SyncHolder<i32> = SyncHolder::default();

        holder.set(Some(1));

        let mut subscription = holder.subscribe();

        assert_eq!(
            subscription
                .wait_until_present(&CancellationToken::new())
                .await,
            SyncHolderPresence::Present
        );
    }

    #[tokio::test]
    async fn wait_until_present_resolves_after_a_later_set() {
        let holder: SyncHolder<i32> = SyncHolder::default();
        let mut subscription = holder.subscribe();
        let cancellation_token = CancellationToken::new();

        let (presence, ()) = tokio::join!(
            subscription.wait_until_present(&cancellation_token),
            async {
                tokio::task::yield_now().await;
                holder.set(Some(3));
            },
        );

        assert_eq!(presence, SyncHolderPresence::Present);
    }

    #[tokio::test]
    async fn wait_until_present_reports_cancellation_before_a_value_arrives() {
        let holder: SyncHolder<i32> = SyncHolder::default();
        let mut subscription = holder.subscribe();
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel();

        assert_eq!(
            subscription.wait_until_present(&cancellation_token).await,
            SyncHolderPresence::Cancelled
        );
    }

    #[test]
    fn changed_stays_pending_after_holder_is_dropped() {
        let holder: SyncHolder<i32> = SyncHolder::default();
        let mut subscription = holder.subscribe();

        drop(holder);

        let mut changed_future = pin!(subscription.changed());
        let mut context = Context::from_waker(Waker::noop());

        assert!(changed_future.as_mut().poll(&mut context).is_pending());
    }
}
