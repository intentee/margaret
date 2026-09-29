use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;

use crate::key_set_poll_failure::KeySetPollFailure;

pub enum KeySetPoll<TLocationFailure> {
    Cancelled,
    Failed(KeySetPollFailure<TLocationFailure>),
    Fetched(AcceptedKeySetDocument),
}
