use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use margaret_websocket_envelope::credit_grant::CreditGrant;

use crate::granted_credit_outcome::GrantedCreditOutcome;

/// How many response frames the peer may still send for one exchange. This is the bound on what
/// that exchange buffers: the peer may not run further ahead than the credit granted to it, and a
/// frame is granted back as the consumer drains one.
#[derive(Clone)]
pub(crate) struct GrantedCredit {
    frames: Arc<AtomicUsize>,
}

impl GrantedCredit {
    pub(crate) fn new(window: CreditGrant) -> Self {
        Self {
            frames: Arc::new(AtomicUsize::new(window.frames())),
        }
    }

    pub(crate) fn grant_one_frame(&self) {
        self.frames.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn spend_one_frame(&self) -> GrantedCreditOutcome {
        match self
            .frames
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |frames| {
                frames.checked_sub(1)
            }) {
            Ok(_) => GrantedCreditOutcome::Spent,
            Err(_) => GrantedCreditOutcome::Exhausted,
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_websocket_envelope::credit_grant::CreditGrant;

    use super::GrantedCredit;
    use super::GrantedCreditOutcome;

    const TWO_FRAMES: CreditGrant = CreditGrant::from_frames(2);

    #[test]
    fn spends_exactly_the_credit_it_granted() {
        let granted = GrantedCredit::new(TWO_FRAMES);

        for _ in 0..TWO_FRAMES.frames() {
            assert_eq!(granted.spend_one_frame(), GrantedCreditOutcome::Spent);
        }

        assert_eq!(granted.spend_one_frame(), GrantedCreditOutcome::Exhausted);
    }

    #[test]
    fn spends_again_once_a_consumed_frame_is_granted_back() {
        let granted = GrantedCredit::new(CreditGrant::from_frames(0));

        assert_eq!(granted.spend_one_frame(), GrantedCreditOutcome::Exhausted);

        granted.grant_one_frame();

        assert_eq!(granted.spend_one_frame(), GrantedCreditOutcome::Spent);
    }
}
