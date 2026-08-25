use crate::schema_identifier::schema_identifier;
use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

/// # Errors
///
/// Returns `SchemaIdentifierNamingError` propagated from the work it performs.
pub fn minimum_constraint_name(
    table: &str,
    column: &str,
) -> Result<String, SchemaIdentifierNamingError> {
    schema_identifier(&[table, column, "minimum"])
}

#[cfg(test)]
mod tests {
    use super::minimum_constraint_name;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn joins_the_table_and_column_with_a_minimum_suffix() {
        assert_eq!(
            minimum_constraint_name("fragment_metadata", "size_payload")
                .expect("the identifier is within the limit"),
            "fragment_metadata_size_payload_minimum"
        );
    }

    #[test]
    fn rejects_a_derived_name_over_the_byte_limit() {
        let column = "a".repeat(MAX_IDENTIFIER_BYTES);

        let error = minimum_constraint_name("fragment_metadata", &column)
            .expect_err("the derived constraint name exceeds the limit");

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } = error;

        assert_eq!(
            length,
            "fragment_metadata".len() + 1 + MAX_IDENTIFIER_BYTES + "_minimum".len()
        );
    }
}
