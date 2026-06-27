use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use tokio::sync::Notify;

use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct Metrics {
    signal: Arc<Notify>,
    sweeps: Arc<AtomicUsize>,
}

impl Metrics {
    #[constructor]
    pub fn create() -> Self {
        Self {
            signal: Arc::new(Notify::new()),
            sweeps: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn record_sweep(&self) {
        self.sweeps.fetch_add(1, Ordering::SeqCst);
        self.signal.notify_one();
    }

    pub fn sweeps(&self) -> usize {
        self.sweeps.load(Ordering::SeqCst)
    }

    pub async fn wait_for_sweep(&self) {
        self.signal.notified().await;
    }
}
