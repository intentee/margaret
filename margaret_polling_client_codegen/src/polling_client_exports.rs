use proc_macro2::TokenStream;

pub struct PollingClientExports {
    pub client: TokenStream,
    pub verifier: TokenStream,
}
