use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct SyntheticRoute {
    pub handler: TokenStream,
    pub label: String,
    pub path: String,
    pub server: String,
}
