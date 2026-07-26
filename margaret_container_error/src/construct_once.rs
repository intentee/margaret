use std::future::Future;
use std::sync::Arc;

use tokio::sync::OnceCell;

use crate::construction_error::ConstructionError;

pub async fn construct_once<Constructed>(
    cell: &OnceCell<Result<Arc<Constructed>, Arc<ConstructionError>>>,
    build: impl Future<Output = Result<Arc<Constructed>, Arc<ConstructionError>>>,
) -> Result<Arc<Constructed>, Arc<ConstructionError>> {
    cell.get_or_init(|| build).await.clone()
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

        let first = construct_once(&cell, async {
            attempts.fetch_add(1, Ordering::SeqCst);

            Ok(Arc::new(7u8))
        })
        .await
        .expect("the first construction succeeds");

        let second = construct_once(&cell, ready(Ok(Arc::new(99u8))))
            .await
            .expect("the cached construction succeeds");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[tokio::test]
    async fn caches_a_failure_and_never_reruns_the_build() {
        let cell: OnceCell<Outcome> = OnceCell::new();
        let attempts = AtomicUsize::new(0);

        let first = construct_once(&cell, async {
            attempts.fetch_add(1, Ordering::SeqCst);

            Err(failure("the key file is missing"))
        })
        .await
        .expect_err("the first construction fails");

        let second = construct_once(&cell, ready(Ok(Arc::new(0u8))))
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
            construct_once(&cell, async {
                attempts.fetch_add(1, Ordering::SeqCst);
                tokio::task::yield_now().await;

                Ok(Arc::new(1u8))
            }),
            construct_once(&cell, ready(Ok(Arc::new(2u8)))),
            construct_once(&cell, ready(Ok(Arc::new(3u8)))),
        );

        let first = first.expect("a construction succeeds");
        let second = second.expect("a construction succeeds");
        let third = third.expect("a construction succeeds");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
        assert!(Arc::ptr_eq(&second, &third));
    }

    #[tokio::test]
    async fn a_cancelled_construction_leaves_the_cell_reusable() {
        let cell: OnceCell<Outcome> = OnceCell::new();

        {
            let building = construct_once(&cell, pending::<Outcome>());
            tokio::pin!(building);

            poll_fn(|context| {
                let _ = building.as_mut().poll(context);

                Poll::Ready(())
            })
            .await;
        }

        let value = construct_once(&cell, ready(Ok(Arc::new(42u8))))
            .await
            .expect("a construction after cancellation succeeds");

        assert_eq!(*value, 42);
    }
}
