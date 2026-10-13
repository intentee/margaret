use std::sync::Arc;

use tokio::time::MissedTickBehavior;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

use crate::constants::audit_interval::AUDIT_INTERVAL;
use crate::singletons::audit_log::AuditLog;

#[scheduled_with_tick_timer(interval = AUDIT_INTERVAL, behavior = MissedTickBehavior::Delay)]
pub struct ShelfAudit {
    audit_log: Arc<AuditLog>,
}

impl ShelfAudit {
    #[must_use]
    #[constructor]
    pub fn create(audit_log: Arc<AuditLog>) -> Self {
        Self { audit_log }
    }

    /// # Errors
    ///
    /// Never returns an error; recording an audit cannot fail.
    #[process]
    pub fn run(&self) -> anyhow::Result<()> {
        self.audit_log.record_audit();

        Ok(())
    }
}
