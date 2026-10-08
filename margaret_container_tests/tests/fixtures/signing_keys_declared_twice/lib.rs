#[singleton]
struct Consumer {
    roller: std::sync::Arc<crate::KeyRoller>,
}

impl Consumer {
    #[constructor]
    fn create(roller: std::sync::Arc<crate::KeyRoller>) -> anyhow::Result<Self> {}
}

#[singleton]
#[stores_signing_keys]
struct FirstKeyStore;

impl margaret::framework::jwks_roller::stores_signing_keys::StoresSigningKeys for FirstKeyStore {}

#[singleton]
#[stores_signing_keys]
struct SecondKeyStore;

impl margaret::framework::jwks_roller::stores_signing_keys::StoresSigningKeys for SecondKeyStore {}

pub struct KeyRoller;
