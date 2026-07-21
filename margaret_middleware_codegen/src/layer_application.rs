use proc_macro2::Ident;

pub struct LayerApplication {
    pub field: Ident,
    pub injects_peer_spiffe_id: bool,
    pub injects_routes: bool,
    pub injects_views: bool,
    pub wrapper: Ident,
}
