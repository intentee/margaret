use margaret_schema_identifier_naming::schema_identifier::SchemaIdentifier;

#[derive(Debug)]
pub struct ResolvedIndex {
    pub columns: Vec<String>,
    pub name: SchemaIdentifier,
}
