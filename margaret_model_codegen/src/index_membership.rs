use margaret_schema_identifier_naming::schema_identifier::SchemaIdentifier;

#[derive(Debug)]
pub enum IndexMembership {
    Derived,
    Named(SchemaIdentifier),
}
