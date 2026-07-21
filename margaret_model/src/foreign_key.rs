use crate::on_delete::OnDelete;

pub struct ForeignKey {
    pub column: String,
    pub on_delete: OnDelete,
    pub references_column: String,
    pub references_table: String,
}
