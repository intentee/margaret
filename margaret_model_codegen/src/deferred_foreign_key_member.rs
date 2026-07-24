use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::column_type::ColumnType;
use margaret_model::on_delete::OnDelete;
use margaret_schema_identifier_naming::schema_identifier::SchemaIdentifier;

pub(crate) struct DeferredForeignKeyMember {
    pub(crate) field_display: String,
    pub(crate) local_column: String,
    pub(crate) local_type: ColumnType,
    pub(crate) name: SchemaIdentifier,
    pub(crate) on_delete: OnDelete,
    pub(crate) reference_display: String,
    pub(crate) referenced_field: String,
    pub(crate) target_path: CanonicalPath,
}
