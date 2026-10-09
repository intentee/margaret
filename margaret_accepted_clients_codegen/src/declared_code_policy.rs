use url::Url;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_oauth_vocabulary::scope::Scope;

use crate::declared_consent::DeclaredConsent;
use crate::declared_id_token_signing::DeclaredIdTokenSigning;

#[derive(Debug)]
pub struct DeclaredCodePolicy {
    pub consent: DeclaredConsent,
    pub id_token_signing: DeclaredIdTokenSigning,
    pub redirect_routes: Vec<CanonicalPath>,
    pub redirect_uris: Vec<Url>,
    pub refresh_token: bool,
    pub scopes: Vec<Scope>,
}
