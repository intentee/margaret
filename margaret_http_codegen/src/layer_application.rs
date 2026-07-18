use proc_macro2::Ident;

pub(crate) struct LayerApplication {
    pub(crate) field: Ident,
    pub(crate) injects_cookie_jar: bool,
    pub(crate) injects_routes: bool,
    pub(crate) wrapper: Ident,
}
