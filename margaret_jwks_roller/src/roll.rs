use std::sync::Arc;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::loaded_secret::LoadedSecret;
use crate::roller_error::RollerError;

fn load_or_fresh(storage: &dyn JwksSecretStorage, curve: Curve) -> Result<JwksSecret, RollerError> {
    match storage.load()? {
        LoadedSecret::Present(loaded) => Ok(*loaded),
        LoadedSecret::Absent => JwksSecret::fresh(curve).map_err(RollerError::KeyGeneration),
    }
}

fn next_secret(
    storage: &dyn JwksSecretStorage,
    holder: &JwksSecretHolder,
    curve: Curve,
) -> Result<JwksSecret, RollerError> {
    match holder.get() {
        Some(current) => current.rotate().map_err(RollerError::KeyGeneration),
        None => load_or_fresh(storage, curve),
    }
}

pub fn roll(
    storage: &dyn JwksSecretStorage,
    holder: &JwksSecretHolder,
    curve: Curve,
) -> Result<(), RollerError> {
    let next = next_secret(storage, holder, curve)?;

    storage.persist(&next)?;
    holder.set(Some(Arc::new(next)));

    Ok(())
}
