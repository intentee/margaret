use std::sync::Arc;

use tokio::time::Instant;

use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[derive(Clone)]
pub struct HeldKeySet {
    pub fetched_at: Instant,
    pub key_set: Arc<VerificationKeySet>,
}
