use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct OpeningHours {
    closes_at: u8,
    opens_at: u8,
}

impl OpeningHours {
    /// # Errors
    ///
    /// Returns an error when the library would close before it opens.
    #[constructor]
    pub fn create(
        #[console_argument(from = "opens-at")] opens_at: u8,
        #[console_argument(from = "closes-at")] closes_at: u8,
    ) -> anyhow::Result<Self> {
        anyhow::ensure!(
            opens_at < closes_at,
            "the library closes at {closes_at} before it opens at {opens_at}"
        );

        Ok(Self {
            closes_at,
            opens_at,
        })
    }

    #[must_use]
    pub fn closes_at(&self) -> u8 {
        self.closes_at
    }

    #[must_use]
    pub fn opens_at(&self) -> u8 {
        self.opens_at
    }
}
