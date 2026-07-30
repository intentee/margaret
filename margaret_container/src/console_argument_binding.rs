use proc_macro2::TokenStream;

pub struct ConsoleArgumentBinding {
    pub slot: usize,
    pub value: TokenStream,
}
