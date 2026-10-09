use crate::key_set_holding::KeySetHolding;

pub struct KeySetSnapshot {
    pub(crate) held_fetches: u64,
    pub holding: KeySetHolding,
    pub(crate) started_fetches: u64,
}
