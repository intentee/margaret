#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameworkEnvironmentVariable {
    JwksSecretStorage,
    OidcProviderStateStorage,
}

impl FrameworkEnvironmentVariable {
    pub const ALL: [Self; 2] = [Self::JwksSecretStorage, Self::OidcProviderStateStorage];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::JwksSecretStorage => "MARGARET_JWKS_SECRET_STORAGE",
            Self::OidcProviderStateStorage => "MARGARET_OIDC_PROVIDER_STATE_STORAGE",
        }
    }
}
