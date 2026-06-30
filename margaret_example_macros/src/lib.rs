use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn traced(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}
