use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub(crate) enum ShapeRead {
    Fallible { field: Ident, read: TokenStream },
    Infallible { field: Ident, value: TokenStream },
}
