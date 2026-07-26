use margaret_jwks_keygen::jwks_secret::JwksSecret;

use crate::loaded_secret::LoadedSecret;
pub trait JwksSecretStorage: Send + Sync {
    fn load(&self) -> anyhow::Result<LoadedSecret>;
    fn persist(&self, secret: &JwksSecret) -> anyhow::Result<()>;
}
