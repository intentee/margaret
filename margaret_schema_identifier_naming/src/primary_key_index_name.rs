use crate::schema_identifier::SchemaIdentifier;
use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

pub fn primary_key_index_name(
    table: &str,
) -> Result<SchemaIdentifier, SchemaIdentifierNamingError> {
    SchemaIdentifier::from_segments(&[table, "pkey"])
}

#[cfg(test)]
mod tests {
    use super::primary_key_index_name;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn appends_the_pkey_suffix_to_the_table() {
        assert_eq!(
            primary_key_index_name("authors")
                .expect("the identifier is within the limit")
                .as_str(),
            "authors_pkey"
        );
    }

    #[test]
    fn rejects_a_derived_name_over_the_byte_limit() {
        let table = "a".repeat(MAX_IDENTIFIER_BYTES);

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } =
            primary_key_index_name(&table).expect_err("the derived name exceeds the limit");

        assert_eq!(length, MAX_IDENTIFIER_BYTES + "_pkey".len());
    }
}
