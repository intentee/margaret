use std::sync::Arc;

use tokio::sync::Barrier;

const STORE_AND_TEST: usize = 2;

#[derive(Clone)]
pub struct SigningKeysSteps {
    barrier: Arc<Barrier>,
}

impl SigningKeysSteps {
    pub async fn intervene(&self, intervention: impl Future<Output = ()>) {
        self.barrier.wait().await;
        intervention.await;
        self.barrier.wait().await;
    }

    pub async fn pass(&self) {
        self.barrier.wait().await;
        self.barrier.wait().await;
    }
}

impl Default for SigningKeysSteps {
    fn default() -> Self {
        Self {
            barrier: Arc::new(Barrier::new(STORE_AND_TEST)),
        }
    }
}
