use std::collections::BTreeSet;

use url::Url;

use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret_oauth_vocabulary::scope::Scope;

use crate::consent_policy::ConsentPolicy;
use crate::non_empty_set::NonEmptySet;
use crate::refresh_token_grant::RefreshTokenGrant;

pub struct CodeGrantPolicy {
    pub consent: ConsentPolicy,
    pub id_token_signing: IdTokenSigning,
    pub redirect_uris: NonEmptySet<Url>,
    pub refresh: RefreshTokenGrant,
    pub scopes: BTreeSet<Scope>,
}
