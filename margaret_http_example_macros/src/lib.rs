use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn can(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}
