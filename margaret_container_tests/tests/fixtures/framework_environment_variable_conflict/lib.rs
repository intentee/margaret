#[singleton]
struct Consumer {
    vault: std::sync::Arc<crate::Vault>,
}

impl Consumer {
    #[constructor]
    fn create(
        vault: std::sync::Arc<crate::Vault>,
        #[environment_variable(from = "VAULT_TOKEN")] token: String,
    ) -> anyhow::Result<Self> {}
}

pub struct Vault;

pub struct VaultToken;
