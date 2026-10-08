use crate::on_delete::OnDelete;

#[derive(Debug, Eq, PartialEq)]
pub struct ForeignKey {
    pub columns: &'static [&'static str],
    pub on_delete: OnDelete,
    pub references_columns: &'static [&'static str],
    pub references_table: &'static str,
}
