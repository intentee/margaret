use margaret_jwks_keygen::jwks_secret::JwksSecret;

use crate::loaded_secret::LoadedSecret;
pub trait JwksSecretStorage: Send + Sync {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    fn load(&self) -> anyhow::Result<LoadedSecret>;
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    fn persist(&self, secret: &JwksSecret) -> anyhow::Result<()>;
}
