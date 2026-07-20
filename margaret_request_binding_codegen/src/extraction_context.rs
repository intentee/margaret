use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub struct ExtractionContext<'context> {
    pub binder_owner: &'context TokenStream,
    pub error_return: &'context TokenStream,
    pub request_local: &'context Ident,
}
