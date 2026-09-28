use margaret_jws_verification::verification_key_set::VerificationKeySet;

use crate::key_set_poll_failure::KeySetPollFailure;

pub enum KeySetPoll<TLocationFailure> {
    Cancelled,
    Failed(KeySetPollFailure<TLocationFailure>),
    Fetched(VerificationKeySet),
}
