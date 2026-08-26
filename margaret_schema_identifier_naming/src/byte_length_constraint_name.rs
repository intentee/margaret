use crate::schema_identifier::schema_identifier;
use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

/// # Errors
///
/// Returns `SchemaIdentifierNamingError` propagated from the work it performs.
pub fn byte_length_constraint_name(
    table: &str,
    column: &str,
) -> Result<String, SchemaIdentifierNamingError> {
    schema_identifier(&[table, column, "byte_length"])
}

#[cfg(test)]
mod tests {
    use super::byte_length_constraint_name;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn joins_the_table_and_column_with_a_byte_length_suffix() {
        assert_eq!(
            byte_length_constraint_name("fragment_metadata", "hash")
                .expect("the identifier is within the limit"),
            "fragment_metadata_hash_byte_length"
        );
    }

    #[test]
    fn rejects_a_derived_name_over_the_byte_limit() {
        let column = "a".repeat(MAX_IDENTIFIER_BYTES);

        let error = byte_length_constraint_name("fragment_metadata", &column)
            .expect_err("the derived constraint name exceeds the limit");

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } = error;

        assert_eq!(
            length,
            "fragment_metadata".len() + 1 + MAX_IDENTIFIER_BYTES + "_byte_length".len()
        );
    }
}
