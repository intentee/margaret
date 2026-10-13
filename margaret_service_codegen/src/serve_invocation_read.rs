use proc_macro2::TokenStream;

pub struct ServeInvocationRead {
    pub binding: TokenStream,
    pub chain: TokenStream,
}
