use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tokio::sync::OnceCell;

use crate::construction_error::ConstructionError;

struct PoisonOnCancel<'flag> {
    poisoned: &'flag AtomicBool,
    armed: bool,
}

impl PoisonOnCancel<'_> {
    fn disarm(mut self) {
        self.armed = false;
    }
}

impl Drop for PoisonOnCancel<'_> {
    fn drop(&mut self) {
        if self.armed {
            self.poisoned.store(true, Ordering::Release);
        }
    }
}

pub struct ConstructionSlot<Constructed: ?Sized> {
    cell: OnceCell<Result<Arc<Constructed>, Arc<ConstructionError>>>,
    poisoned: AtomicBool,
}

impl<Constructed: ?Sized> ConstructionSlot<Constructed> {
    pub async fn construct_once(
        &self,
        singleton: &'static str,
        build: impl Future<Output = Result<Arc<Constructed>, Arc<ConstructionError>>>,
    ) -> Result<Arc<Constructed>, Arc<ConstructionError>> {
        self.cell
            .get_or_init(|| async {
                if self.poisoned.load(Ordering::Acquire) {
                    return Err(Arc::new(ConstructionError::Interrupted { singleton }));
                }

                let guard = PoisonOnCancel {
                    poisoned: &self.poisoned,
                    armed: true,
                };
                let outcome = build.await;

                guard.disarm();

                outcome
            })
            .await
            .clone()
    }
}

impl<Constructed: ?Sized> Default for ConstructionSlot<Constructed> {
    fn default() -> Self {
        Self {
            cell: OnceCell::new(),
            poisoned: AtomicBool::new(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::future::poll_fn;
    use std::future::ready;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::task::Poll;

    use super::ConstructionSlot;
    use crate::construction_error::ConstructionError;

    type Outcome = Result<Arc<u8>, Arc<ConstructionError>>;
    type Build<'future> = Pin<Box<dyn Future<Output = Outcome> + 'future>>;

    fn boxed<'future>(future: impl Future<Output = Outcome> + 'future) -> Build<'future> {
        Box::pin(future)
    }

    fn failure(message: &'static str) -> Arc<ConstructionError> {
        Arc::new(ConstructionError::UserError {
            singleton: "crate::keys::KeyLoader",
            source: anyhow::anyhow!(message),
        })
    }

    #[tokio::test]
    async fn runs_the_build_once_and_caches_the_constructed_value() {
        let slot: ConstructionSlot<u8> = ConstructionSlot::default();
        let attempts = AtomicUsize::new(0);

        let first = slot
            .construct_once(
                "crate::keys::KeyLoader",
                boxed(async {
                    attempts.fetch_add(1, Ordering::SeqCst);

                    Ok(Arc::new(7u8))
                }),
            )
            .await
            .expect("the first construction succeeds");

        let second = slot
            .construct_once("crate::keys::KeyLoader", boxed(ready(Ok(Arc::new(99u8)))))
            .await
            .expect("the cached construction succeeds");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[tokio::test]
    async fn caches_a_failure_and_never_reruns_the_build() {
        let slot: ConstructionSlot<u8> = ConstructionSlot::default();
        let attempts = AtomicUsize::new(0);

        let first = slot
            .construct_once(
                "crate::keys::KeyLoader",
                boxed(async {
                    attempts.fetch_add(1, Ordering::SeqCst);

                    Err(failure("the key file is missing"))
                }),
            )
            .await
            .expect_err("the first construction fails");

        let second = slot
            .construct_once("crate::keys::KeyLoader", boxed(ready(Ok(Arc::new(0u8)))))
            .await
            .expect_err("the cached failure is returned again");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
        assert!(second.to_string().contains("the key file is missing"));
    }

    #[tokio::test]
    async fn serves_every_concurrent_waiter_a_single_construction() {
        let slot: ConstructionSlot<u8> = ConstructionSlot::default();
        let attempts = AtomicUsize::new(0);

        let (first, second, third) = tokio::join!(
            slot.construct_once(
                "crate::keys::KeyLoader",
                boxed(async {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    tokio::task::yield_now().await;

                    Ok(Arc::new(1u8))
                }),
            ),
            slot.construct_once("crate::keys::KeyLoader", boxed(ready(Ok(Arc::new(2u8))))),
            slot.construct_once("crate::keys::KeyLoader", boxed(ready(Ok(Arc::new(3u8))))),
        );

        let first = first.expect("a construction succeeds");
        let second = second.expect("a construction succeeds");
        let third = third.expect("a construction succeeds");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
        assert!(Arc::ptr_eq(&second, &third));
    }

    fn signals_entry_then_never_completes(entered: Arc<AtomicBool>) -> Build<'static> {
        let mut warmed = false;

        boxed(poll_fn(move |context| {
            if warmed {
                entered.store(true, Ordering::SeqCst);
            } else {
                warmed = true;
                context.waker().wake_by_ref();
            }

            Poll::Pending
        }))
    }

    async fn drive_until_construction_entered<Building>(building: &mut Building, entered: &AtomicBool)
    where
        Building: Future<Output = Outcome> + Unpin,
    {
        poll_fn(|context| {
            let _ = Pin::new(&mut *building).poll(context);

            if entered.load(Ordering::SeqCst) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    #[tokio::test]
    async fn a_cancelled_construction_poisons_the_slot_against_reruns() {
        let slot: ConstructionSlot<u8> = ConstructionSlot::default();
        let entered = Arc::new(AtomicBool::new(false));

        let mut building = Box::pin(slot.construct_once(
            "crate::keys::KeyLoader",
            signals_entry_then_never_completes(entered.clone()),
        ));

        drive_until_construction_entered(&mut building, &entered).await;

        drop(building);

        let error = slot
            .construct_once("crate::keys::KeyLoader", boxed(ready(Ok(Arc::new(42u8)))))
            .await
            .expect_err("a cancelled construction refuses to re-run");

        assert!(error.to_string().contains("interrupted"));
    }

    #[tokio::test]
    async fn a_cancelled_construction_poisons_a_concurrent_waiter() {
        let slot: ConstructionSlot<u8> = ConstructionSlot::default();
        let entered = Arc::new(AtomicBool::new(false));

        let mut cancelled = Box::pin(slot.construct_once(
            "crate::keys::KeyLoader",
            signals_entry_then_never_completes(entered.clone()),
        ));
        let mut waiter = Box::pin(
            slot.construct_once("crate::keys::KeyLoader", boxed(ready(Ok(Arc::new(1u8))))),
        );

        drive_until_construction_entered(&mut cancelled, &entered).await;

        poll_fn(|context| {
            assert!(waiter.as_mut().poll(context).is_pending());

            Poll::Ready(())
        })
        .await;

        drop(cancelled);

        let error = waiter
            .await
            .expect_err("the waiter refuses to re-run the cancelled construction");

        assert!(error.to_string().contains("interrupted"));
    }
}
