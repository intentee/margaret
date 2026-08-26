use proc_macro2::TokenStream;

pub struct ServeInputBinding {
    pub slot: usize,
    pub value: TokenStream,
}
