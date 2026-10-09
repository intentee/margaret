use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;

use crate::key_set_poll_failure::KeySetPollFailure;

pub(crate) enum KeySetPoll {
    Cancelled,
    Failed(KeySetPollFailure),
    Fetched(AcceptedKeySetDocument),
}
