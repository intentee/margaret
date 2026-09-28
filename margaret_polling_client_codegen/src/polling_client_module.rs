use proc_macro2::TokenStream;

pub struct PollingClientModule {
    pub exports: TokenStream,
    pub segment: String,
}
