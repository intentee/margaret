use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub struct ExtractionContext<'context> {
    pub continuation_return: &'context TokenStream,
    pub error_return: &'context TokenStream,
    pub provider_access: &'context TokenStream,
    pub request_local: &'context Ident,
    pub response_return: &'context TokenStream,
}
