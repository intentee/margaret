use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub(crate) struct BoundRead {
    pub(crate) read: TokenStream,
    pub(crate) value: Ident,
}
