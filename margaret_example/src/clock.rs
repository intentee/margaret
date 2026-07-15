use std::time::SystemTimeError;

pub trait Clock: Send + Sync {
    fn now(&self) -> Result<u64, SystemTimeError>;
}
