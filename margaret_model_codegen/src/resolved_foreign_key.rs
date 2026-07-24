use margaret_model::on_delete::OnDelete;
use margaret_schema_identifier_naming::schema_identifier::SchemaIdentifier;

#[derive(Debug)]
pub struct ResolvedForeignKey {
    pub columns: Vec<String>,
    pub name: SchemaIdentifier,
    pub on_delete: OnDelete,
    pub references_columns: Vec<String>,
    pub references_table: String,
}
