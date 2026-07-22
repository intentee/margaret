use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct ResolvedForeignKey {
    pub column: String,
    pub on_delete: TokenStream,
    pub references_column: String,
    pub references_table: String,
    pub unique: bool,
}
