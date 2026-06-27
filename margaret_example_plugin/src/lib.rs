use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct BuildInfo;

impl BuildInfo {
    #[constructor]
    pub fn create() -> Self {
        Self
    }

    pub fn revision(&self) -> &'static str {
        "1.0.0"
    }
}
