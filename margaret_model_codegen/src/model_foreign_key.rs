use margaret_model::on_delete::OnDelete;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelForeignKey {
    pub columns: Vec<String>,
    pub field: String,
    pub on_delete: OnDelete,
    pub references_columns: Vec<String>,
    pub references_table: String,
}
