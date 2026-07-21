use proc_macro2::TokenStream;

pub(crate) struct ResolvedForeignKey {
    pub(crate) columns: Vec<String>,
    pub(crate) on_delete: TokenStream,
    pub(crate) references_columns: Vec<String>,
    pub(crate) references_table: String,
    pub(crate) unique: bool,
}
