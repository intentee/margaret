use std::future::Future;

use crate::joined_route_parameter_bindings::JoinedRouteParameterBindings;

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub async fn join_route_parameter_bindings<First, Second, FirstModel, SecondModel, Error>(
    first: First,
    second: Second,
) -> Result<JoinedRouteParameterBindings<FirstModel, SecondModel>, Error>
where
    First: Future<Output = Result<FirstModel, Error>>,
    Second: Future<Output = Result<SecondModel, Error>>,
{
    let (first, second) = tokio::try_join!(first, second)?;

    Ok(JoinedRouteParameterBindings { first, second })
}

#[cfg(test)]
mod tests {
    use std::future::poll_fn;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::task::Poll;

    use super::join_route_parameter_bindings;

    struct DropSignal(Arc<AtomicBool>);

    impl Drop for DropSignal {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn returns_both_successful_values() {
        let joined =
            join_route_parameter_bindings(async { Ok::<_, &'static str>(1) }, async { Ok(2) })
                .await
                .expect("both bindings resolve");

        assert_eq!(joined.first, 1);
        assert_eq!(joined.second, 2);
    }

    #[tokio::test]
    async fn polls_both_bindings_concurrently_and_drops_unfinished_work_after_an_error() {
        let first_polled = Arc::new(AtomicBool::new(false));
        let first_dropped = Arc::new(AtomicBool::new(false));
        let first = {
            let first_polled = Arc::clone(&first_polled);
            let drop_signal = DropSignal(Arc::clone(&first_dropped));

            poll_fn(move |_| {
                let _drop_signal = &drop_signal;
                first_polled.store(true, Ordering::SeqCst);
                Poll::<Result<u8, &'static str>>::Pending
            })
        };
        let second = {
            let first_polled = Arc::clone(&first_polled);

            poll_fn(move |_| {
                assert!(first_polled.load(Ordering::SeqCst));
                Poll::Ready(Err::<u8, _>("database unavailable"))
            })
        };

        let error = join_route_parameter_bindings(first, second)
            .await
            .err()
            .expect("the failing binding is reported");

        assert_eq!(error, "database unavailable");
        assert!(first_dropped.load(Ordering::SeqCst));
    }
}
