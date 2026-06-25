use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub(crate) struct LayerApplication {
    pub(crate) marker_value: TokenStream,
    pub(crate) middleware_field: Ident,
    pub(crate) priority: i64,
}
