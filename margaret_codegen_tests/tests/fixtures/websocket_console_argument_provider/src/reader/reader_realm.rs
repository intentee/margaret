use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct ReaderRealm {
    realm: String,
}

impl ReaderRealm {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(#[console_argument(from = "realm")] realm: String) -> anyhow::Result<Self> {
        Ok(Self { realm })
    }

    #[must_use]
    pub fn admits(&self, reader: &str) -> bool {
        reader.starts_with(&self.realm)
    }
}
