pub struct ForeignKey {
    pub columns: Vec<String>,
    pub references_columns: Vec<String>,
    pub references_table: String,
}
