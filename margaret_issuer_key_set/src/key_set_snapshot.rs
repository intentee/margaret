use crate::key_set_holding::KeySetHolding;

pub struct KeySetSnapshot {
    pub holding: KeySetHolding,
    pub(crate) started_fetches: u64,
}
