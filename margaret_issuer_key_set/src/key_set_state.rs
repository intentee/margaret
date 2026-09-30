use crate::key_set_holding::KeySetHolding;
use crate::key_set_polling::KeySetPolling;

pub(crate) struct KeySetState {
    pub(crate) completed_fetches: u64,
    pub(crate) holding: KeySetHolding,
    pub(crate) polling: KeySetPolling,
    pub(crate) started_fetches: u64,
}
