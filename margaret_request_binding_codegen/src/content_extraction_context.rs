use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub struct ContentExtractionContext<'context> {
    pub body_local: &'context Ident,
    pub content_local: &'context Ident,
    pub continuation_return: &'context TokenStream,
    pub limit: u64,
    pub request_local: &'context Ident,
    pub system_error_return: &'context TokenStream,
}
