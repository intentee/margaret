use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

pub(crate) struct BoxedService {
    inner: Box<dyn Service>,
}

impl BoxedService {
    pub(crate) fn new(inner: Box<dyn Service>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl Service for BoxedService {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        self.inner.run(cancellation_token).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use anyhow::Result;
    use async_trait::async_trait;
    use tokio_util::sync::CancellationToken;
    use trzcina::Service;

    use super::BoxedService;

    struct RecordingService {
        ran: Arc<AtomicBool>,
    }

    #[async_trait]
    impl Service for RecordingService {
        fn name(&self) -> &'static str {
            "recording"
        }

        async fn run(self: Box<Self>, _cancellation_token: CancellationToken) -> Result<()> {
            self.ran.store(true, Ordering::SeqCst);

            Ok(())
        }
    }

    #[tokio::test]
    async fn runs_the_boxed_service_under_its_own_name() {
        let ran = Arc::new(AtomicBool::new(false));
        let boxed = Box::new(BoxedService::new(Box::new(RecordingService {
            ran: Arc::clone(&ran),
        })));

        assert_eq!(boxed.name(), "recording");
        boxed
            .run(CancellationToken::new())
            .await
            .expect("the boxed service runs");
        assert!(ran.load(Ordering::SeqCst));
    }
}
