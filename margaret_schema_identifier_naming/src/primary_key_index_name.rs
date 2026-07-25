use crate::schema_identifier::schema_identifier;
use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

pub fn primary_key_index_name(table: &str) -> Result<String, SchemaIdentifierNamingError> {
    schema_identifier(&[table, "pkey"])
}

#[cfg(test)]
mod tests {
    use super::primary_key_index_name;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn appends_the_pkey_suffix_to_the_table() {
        assert_eq!(
            primary_key_index_name("authors").expect("the identifier is within the limit"),
            "authors_pkey"
        );
    }

    #[test]
    fn rejects_a_derived_name_over_the_byte_limit() {
        let table = "a".repeat(MAX_IDENTIFIER_BYTES);

        let error = primary_key_index_name(&table)
            .expect_err("the derived primary key index name exceeds the limit");

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } = error;

        assert_eq!(length, MAX_IDENTIFIER_BYTES + "_pkey".len());
    }
}
