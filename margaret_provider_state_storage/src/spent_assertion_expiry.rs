use std::time::Duration;
use std::time::Instant;

use moka::Expiry;

use crate::spent_assertion::SpentAssertion;
use crate::spent_assertion_key::SpentAssertionKey;

pub(crate) struct SpentAssertionExpiry;

impl Expiry<SpentAssertionKey, SpentAssertion> for SpentAssertionExpiry {
    fn expire_after_create(
        &self,
        _key: &SpentAssertionKey,
        spent: &SpentAssertion,
        _created_at: Instant,
    ) -> Option<Duration> {
        Some(spent.retained_for)
    }

    fn expire_after_update(
        &self,
        _key: &SpentAssertionKey,
        spent: &SpentAssertion,
        _updated_at: Instant,
        _duration_until_expiry: Option<Duration>,
    ) -> Option<Duration> {
        Some(spent.retained_for)
    }
}
