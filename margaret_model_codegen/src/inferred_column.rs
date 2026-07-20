use proc_macro2::TokenStream;

pub(crate) struct InferredColumn {
    pub(crate) column_type: TokenStream,
    pub(crate) default: TokenStream,
    pub(crate) nullable: bool,
}
