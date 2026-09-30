use std::collections::BTreeSet;

use margaret_jwks_secret_store::provider_identity::ProviderIdentity;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::audience::Audience;

pub(crate) struct TokenIssue {
    pub(crate) identity: ProviderIdentity,
    pub(crate) issued_token_type: Option<String>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) resource: Audience,
    pub(crate) scopes: BTreeSet<Scope>,
    pub(crate) subject: String,
}
