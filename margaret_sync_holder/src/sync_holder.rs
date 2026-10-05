use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::sync::Arc;

use tokio::sync::watch;
use tokio::sync::watch::Sender;

use crate::sync_holder_subscription::SyncHolderSubscription;

pub struct SyncHolder<TItem> {
    sender: Arc<Sender<TItem>>,
}

impl<TItem: Clone + Send + Sync + 'static> SyncHolder<TItem> {
    #[must_use]
    pub fn new(item: TItem) -> Self {
        let (sender, _initial_receiver) = watch::channel(item);

        Self {
            sender: Arc::new(sender),
        }
    }

    #[must_use]
    pub fn get(&self) -> TItem {
        self.sender.borrow().clone()
    }

    pub fn set(&self, item: TItem) {
        self.sender.send_replace(item);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<TItem> {
        SyncHolderSubscription::new(self.sender.subscribe())
    }
}

impl<TItem> Clone for SyncHolder<TItem> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<TItem> Debug for SyncHolder<TItem> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_struct("SyncHolder").finish_non_exhaustive()
    }
}

impl<TItem: Clone + Default + Send + Sync + 'static> Default for SyncHolder<TItem> {
    fn default() -> Self {
        Self::new(TItem::default())
    }
}

#[cfg(test)]
mod tests {
    use super::SyncHolder;

    #[test]
    fn get_returns_none_before_set() {
        let holder: SyncHolder<Option<i32>> = SyncHolder::default();

        assert_eq!(holder.get(), None);
    }

    #[test]
    fn set_then_get_returns_value() {
        let holder: SyncHolder<Option<i32>> = SyncHolder::default();

        holder.set(Some(42));

        assert_eq!(holder.get(), Some(42));
    }

    #[test]
    fn clone_shares_state_with_original() {
        let holder: SyncHolder<Option<i32>> = SyncHolder::default();
        let cloned = holder.clone();

        holder.set(Some(7));

        assert_eq!(cloned.get(), Some(7));
    }

    #[test]
    fn get_returns_the_initial_value() {
        assert_eq!(SyncHolder::new(3).get(), 3);
    }

    #[test]
    fn debug_format_uses_non_exhaustive_struct() {
        let holder: SyncHolder<Option<i32>> = SyncHolder::default();

        assert_eq!(format!("{holder:?}"), "SyncHolder { .. }");
    }
}
