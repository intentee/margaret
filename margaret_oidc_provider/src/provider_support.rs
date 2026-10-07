use margaret_oauth_vocabulary::grant_type::GrantType;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderSupport {
    pub grant_types: &'static [GrantType],
    pub scopes: &'static [&'static str],
}
