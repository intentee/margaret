use crate::schema_identifier::schema_identifier;
use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

pub fn index_name(table: &str, column: &str) -> Result<String, SchemaIdentifierNamingError> {
    schema_identifier(&[table, column, "index"])
}

#[cfg(test)]
mod tests {
    use super::index_name;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn joins_the_table_and_column_with_an_index_suffix() {
        assert_eq!(
            index_name("articles", "created_at").expect("the identifier is within the limit"),
            "articles_created_at_index"
        );
    }

    #[test]
    fn preserves_underscores_in_the_column() {
        assert_eq!(
            index_name("articles", "author_id").expect("the identifier is within the limit"),
            "articles_author_id_index"
        );
    }

    #[test]
    fn is_stable_across_repeated_calls() {
        assert_eq!(
            index_name("articles", "created_at").expect("the identifier is within the limit"),
            index_name("articles", "created_at").expect("the identifier is within the limit")
        );
    }

    #[test]
    fn rejects_a_derived_name_over_the_byte_limit() {
        let column = "a".repeat(MAX_IDENTIFIER_BYTES);

        let error = index_name("articles", &column)
            .expect_err("the derived index name exceeds the limit");

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } = error;

        assert_eq!(length, "articles".len() + 1 + MAX_IDENTIFIER_BYTES + "_index".len());
    }
}
