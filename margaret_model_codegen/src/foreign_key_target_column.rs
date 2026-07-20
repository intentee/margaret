use proc_macro2::TokenStream;

pub(crate) struct ForeignKeyTargetColumn {
    pub(crate) column_type: TokenStream,
    pub(crate) name: String,
}
