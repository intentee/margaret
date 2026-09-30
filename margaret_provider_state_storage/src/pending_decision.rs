use uuid::Uuid;

use crate::pending_verdict::PendingVerdict;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingDecision {
    pub subject: Uuid,
    pub verdict: PendingVerdict,
}
