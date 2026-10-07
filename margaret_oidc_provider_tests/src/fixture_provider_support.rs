use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oidc_provider::provider_support::ProviderSupport;

pub const FIXTURE_PROVIDER_SUPPORT: ProviderSupport = ProviderSupport {
    grant_types: &GrantType::ALL,
    scopes: &["artifacts:read", "openid", "profile"],
};
