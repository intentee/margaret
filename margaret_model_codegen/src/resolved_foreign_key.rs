pub(crate) struct ResolvedForeignKey {
    pub(crate) columns: Vec<String>,
    pub(crate) references_columns: Vec<String>,
    pub(crate) references_table: String,
}
