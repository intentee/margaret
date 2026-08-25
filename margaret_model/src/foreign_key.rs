use crate::on_delete::OnDelete;

pub struct ForeignKey {
    pub columns: Vec<String>,
    pub on_delete: OnDelete,
    pub references_columns: Vec<String>,
    pub references_table: String,
}
