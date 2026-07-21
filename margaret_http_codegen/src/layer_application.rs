use proc_macro2::Ident;

pub(crate) struct LayerApplication {
    pub(crate) field: Ident,
    pub(crate) injects_peer_spiffe_id: bool,
    pub(crate) injects_routes: bool,
    pub(crate) injects_views: bool,
    pub(crate) wrapper: Ident,
}
