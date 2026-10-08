use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyRetention {
    pub refresh: Duration,
    pub token: Duration,
}
