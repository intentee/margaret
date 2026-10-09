use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;

pub struct PublishedEndpoints {
    pub authorization: &'static str,
    pub provider: ProviderEndpoints,
}
