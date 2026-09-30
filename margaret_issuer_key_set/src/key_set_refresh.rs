use crate::key_set_holding::KeySetHolding;

pub enum KeySetRefresh {
    PollingStopped,
    Refreshed(KeySetHolding),
}
