use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

pub struct LayerApplication {
    pub concrete: CanonicalPath,
    pub field: Ident,
    pub injects_peer_spiffe_id: bool,
    pub injects_routes: bool,
    pub injects_views: bool,
    pub observes_cancellation: bool,
    pub wrapper: Ident,
}
