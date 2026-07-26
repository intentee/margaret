use std::future::Future;
use std::sync::Arc;

use tokio::sync::OnceCell;

use crate::construction_error::ConstructionError;

struct InterruptionGuard<'cell, Constructed: ?Sized> {
    cell: &'cell OnceCell<Result<Arc<Constructed>, Arc<ConstructionError>>>,
    singleton: &'static str,
    armed: bool,
}

impl<Constructed: ?Sized> InterruptionGuard<'_, Constructed> {
    fn disarm(mut self) {
        self.armed = false;
    }
}

impl<Constructed: ?Sized> Drop for InterruptionGuard<'_, Constructed> {
    fn drop(&mut self) {
        if self.armed {
            let _ = self.cell.set(Err(Arc::new(ConstructionError::Interrupted {
                singleton: self.singleton,
            })));
        }
    }
}

pub async fn construct_once<Constructed: ?Sized>(
    cell: &OnceCell<Result<Arc<Constructed>, Arc<ConstructionError>>>,
    singleton: &'static str,
    build: impl Future<Output = Result<Arc<Constructed>, Arc<ConstructionError>>>,
) -> Result<Arc<Constructed>, Arc<ConstructionError>> {
    let interruption = InterruptionGuard {
        cell,
        singleton,
        armed: true,
    };
    let outcome = cell.get_or_init(|| build).await.clone();

    interruption.disarm();

    outcome
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::future::pending;
    use std::future::poll_fn;
    use std::future::ready;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::task::Poll;

    use tokio::sync::OnceCell;

    use super::construct_once;
    use crate::construction_error::ConstructionError;

    type Outcome = Result<Arc<u8>, Arc<ConstructionError>>;

    fn failure(message: &'static str) -> Arc<ConstructionError> {
        Arc::new(ConstructionError::UserError {
            singleton: "crate::keys::KeyLoader",
            source: anyhow::anyhow!(message),
        })
    }

    #[tokio::test]
    async fn runs_the_build_once_and_caches_the_constructed_value() {
        let cell: OnceCell<Outcome> = OnceCell::new();
        let attempts = AtomicUsize::new(0);

        let first = construct_once(&cell, "crate::keys::KeyLoader", async {
            attempts.fetch_add(1, Ordering::SeqCst);

            Ok(Arc::new(7u8))
        })
        .await
        .expect("the first construction succeeds");

        let second = construct_once(&cell, "crate::keys::KeyLoader", ready(Ok(Arc::new(99u8))))
            .await
            .expect("the cached construction succeeds");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[tokio::test]
    async fn caches_a_failure_and_never_reruns_the_build() {
        let cell: OnceCell<Outcome> = OnceCell::new();
        let attempts = AtomicUsize::new(0);

        let first = construct_once(&cell, "crate::keys::KeyLoader", async {
            attempts.fetch_add(1, Ordering::SeqCst);

            Err(failure("the key file is missing"))
        })
        .await
        .expect_err("the first construction fails");

        let second = construct_once(&cell, "crate::keys::KeyLoader", ready(Ok(Arc::new(0u8))))
            .await
            .expect_err("the cached failure is returned again");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
        assert!(second.to_string().contains("the key file is missing"));
    }

    #[tokio::test]
    async fn serves_every_concurrent_waiter_a_single_construction() {
        let cell: OnceCell<Outcome> = OnceCell::new();
        let attempts = AtomicUsize::new(0);

        let (first, second, third) = tokio::join!(
            construct_once(&cell, "crate::keys::KeyLoader", async {
                attempts.fetch_add(1, Ordering::SeqCst);
                tokio::task::yield_now().await;

                Ok(Arc::new(1u8))
            }),
            construct_once(&cell, "crate::keys::KeyLoader", ready(Ok(Arc::new(2u8)))),
            construct_once(&cell, "crate::keys::KeyLoader", ready(Ok(Arc::new(3u8)))),
        );

        let first = first.expect("a construction succeeds");
        let second = second.expect("a construction succeeds");
        let third = third.expect("a construction succeeds");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
        assert!(Arc::ptr_eq(&second, &third));
    }

    #[tokio::test]
    async fn a_cancelled_construction_poisons_the_cell_against_reruns() {
        let cell: OnceCell<Outcome> = OnceCell::new();

        {
            let building = construct_once(&cell, "crate::keys::KeyLoader", pending::<Outcome>());
            tokio::pin!(building);

            poll_fn(|context| {
                let _ = building.as_mut().poll(context);

                Poll::Ready(())
            })
            .await;
        }

        let error = construct_once(&cell, "crate::keys::KeyLoader", ready(Ok(Arc::new(42u8))))
            .await
            .expect_err("a cancelled construction refuses to re-run");

        assert!(error.to_string().contains("interrupted"));
    }
}
