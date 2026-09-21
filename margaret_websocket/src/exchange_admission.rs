use std::sync::Arc;

use tokio::sync::Semaphore;

pub(crate) enum ExchangeAdmission {
    Admitted(Arc<Semaphore>),
    AlreadyOpen,
}
