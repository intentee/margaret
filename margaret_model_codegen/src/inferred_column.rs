use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct InferredColumn {
    pub column_type: TokenStream,
    pub default: TokenStream,
    pub nullable: bool,
}
