use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;

#[derive(Clone)]
pub struct AuthenticatedUserApplication {
    pub challenge: AuthenticatedUserChallenge,
    pub concrete: CanonicalPath,
    pub field: String,
    pub injects_peer_spiffe_id: bool,
    pub injects_routes: bool,
    pub injects_views: bool,
    pub model: CanonicalPath,
    pub wrapper: Ident,
}
