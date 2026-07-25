use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

pub struct RecordingService {
    ran: Arc<AtomicBool>,
}

impl RecordingService {
    #[must_use]
    pub fn new(ran: Arc<AtomicBool>) -> Self {
        Self { ran }
    }
}

#[async_trait]
impl Service for RecordingService {
    fn name(&self) -> &'static str {
        "recording_service"
    }

    async fn run(self: Box<Self>, _cancellation_token: CancellationToken) -> Result<()> {
        self.ran.store(true, Ordering::SeqCst);

        Ok(())
    }
}
