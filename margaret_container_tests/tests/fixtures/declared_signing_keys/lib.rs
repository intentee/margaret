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
struct KeyStore;

impl margaret::framework::jwks_roller::stores_signing_keys::StoresSigningKeys for KeyStore {}

pub struct KeyRoller;
