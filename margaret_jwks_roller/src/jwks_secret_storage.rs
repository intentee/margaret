use margaret_jwks_keygen::jwks_secret::JwksSecret;

use crate::loaded_secret::LoadedSecret;
use crate::roller_error::RollerError;

pub trait JwksSecretStorage: Send + Sync {
    fn load(&self) -> Result<LoadedSecret, RollerError>;
    fn persist(&self, secret: &JwksSecret) -> Result<(), RollerError>;
}
