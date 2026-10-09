use margaret_model::on_delete::OnDelete;

#[derive(Debug)]
pub struct ResolvedForeignKey {
    pub columns: Vec<String>,
    pub on_delete: OnDelete,
    pub references_columns: Vec<String>,
    pub references_table: String,
}
