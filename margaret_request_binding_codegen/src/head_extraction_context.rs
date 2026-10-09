use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub struct HeadExtractionContext<'context> {
    pub continuation_return: &'context TokenStream,
    pub error_return: &'context TokenStream,
    pub owner: &'context TokenStream,
    pub request_local: &'context Ident,
}
