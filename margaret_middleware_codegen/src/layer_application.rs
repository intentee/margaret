use proc_macro2::Ident;

pub struct LayerApplication {
    pub field: Ident,
    pub injects_routes: bool,
    pub wrapper: Ident,
}
