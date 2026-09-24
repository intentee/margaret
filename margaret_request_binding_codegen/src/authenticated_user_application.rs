use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Clone)]
pub struct AuthenticatedUserApplication {
    pub concrete: CanonicalPath,
    pub field: String,
    pub injects_peer_spiffe_id: bool,
    pub injects_routes: bool,
    pub injects_views: bool,
    pub model: CanonicalPath,
    pub wrapper: Ident,
}
