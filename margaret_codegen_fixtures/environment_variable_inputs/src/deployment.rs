use std::path::PathBuf;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct Deployment {
    pub database_url: String,
    pub upload_root: PathBuf,
}

impl Deployment {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[environment_variable(from = "MARGARET_FIXTURE_DATABASE_URL")] database_url: String,
        #[environment_variable(from = "MARGARET_FIXTURE_UPLOAD_ROOT")] upload_root: PathBuf,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            database_url,
            upload_root,
        })
    }
}
