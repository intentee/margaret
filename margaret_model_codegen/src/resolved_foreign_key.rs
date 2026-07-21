use proc_macro2::TokenStream;

pub(crate) struct ResolvedForeignKey {
    pub(crate) column: String,
    pub(crate) on_delete: TokenStream,
    pub(crate) references_column: String,
    pub(crate) references_table: String,
    pub(crate) unique: bool,
}
