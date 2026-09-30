use std::collections::BTreeSet;

use url::Url;

use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::audience::Audience;

use crate::accepted_client_authentication::AcceptedClientAuthentication;
use crate::consent_policy::ConsentPolicy;
use crate::grant_type::GrantType;
use crate::introspection_permission::IntrospectionPermission;

pub struct AcceptedClient {
    pub authentication: AcceptedClientAuthentication,
    pub client_id: ClientId,
    pub consent: ConsentPolicy,
    pub grants: BTreeSet<GrantType>,
    pub id_token_signing: IdTokenSigning,
    pub introspection: IntrospectionPermission,
    pub redirect_uris: BTreeSet<Url>,
    pub resources: BTreeSet<Audience>,
    pub scopes: BTreeSet<Scope>,
}
