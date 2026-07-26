use proc_macro2::TokenStream;

pub enum AccessorFailure {
    Propagate,
    Report(TokenStream),
}
