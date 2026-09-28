use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::oidc_token_verifier_field::OidcTokenVerifierField;

#[derive(Clone)]
pub struct AuthenticatedUserApplication {
    pub challenge: AuthenticatedUserChallenge,
    pub concrete: CanonicalPath,
    pub field: String,
    pub injects_peer_spiffe_id: bool,
    pub injects_routes: bool,
    pub injects_views: bool,
    pub model: CanonicalPath,
    pub oidc_token_verifiers: Vec<OidcTokenVerifierField>,
    pub wrapper: Ident,
}
