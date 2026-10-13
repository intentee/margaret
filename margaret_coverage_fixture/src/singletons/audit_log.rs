use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct AuditLog {
    audits: AtomicUsize,
}

impl AuditLog {
    #[must_use]
    #[constructor]
    pub fn create() -> Self {
        Self {
            audits: AtomicUsize::new(0),
        }
    }

    #[must_use]
    pub fn audits(&self) -> usize {
        self.audits.load(Ordering::SeqCst)
    }

    pub fn record_audit(&self) {
        self.audits.fetch_add(1, Ordering::SeqCst);
    }
}
