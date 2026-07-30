use crate::schema_identifier::schema_identifier;
use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

/// # Errors
///
/// Returns `SchemaIdentifierNamingError` propagated from the work it performs.
pub fn unique_index_name(
    table: &str,
    columns: &[String],
) -> Result<String, SchemaIdentifierNamingError> {
    let mut segments: Vec<&str> = Vec::with_capacity(columns.len() + 2);

    segments.push(table);
    segments.extend(columns.iter().map(String::as_str));
    segments.push("key");

    schema_identifier(&segments)
}

#[cfg(test)]
mod tests {
    use super::unique_index_name;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn appends_the_key_suffix_to_a_single_column() {
        assert_eq!(
            unique_index_name("authors", &["name".to_string()])
                .expect("the identifier is within the limit"),
            "authors_name_key"
        );
    }

    #[test]
    fn joins_every_column_before_the_key_suffix() {
        assert_eq!(
            unique_index_name("line_items", &["region".to_string(), "number".to_string()])
                .expect("the identifier is within the limit"),
            "line_items_region_number_key"
        );
    }

    #[test]
    fn rejects_a_derived_name_over_the_byte_limit() {
        let column = "a".repeat(MAX_IDENTIFIER_BYTES);

        let error = unique_index_name("line_items", &[column])
            .expect_err("the derived unique index name exceeds the limit");

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } = error;

        assert_eq!(
            length,
            "line_items".len() + 1 + MAX_IDENTIFIER_BYTES + "_key".len()
        );
    }
}
