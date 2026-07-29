use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;
use crate::validate_identifier_length::validate_identifier_length;

/// # Errors
///
/// Returns `SchemaIdentifierNamingError` propagated from the work it performs.
pub fn schema_identifier(segments: &[&str]) -> Result<String, SchemaIdentifierNamingError> {
    let identifier = segments.join("_");

    validate_identifier_length(&identifier)?;

    Ok(identifier)
}

#[cfg(test)]
mod tests {
    use super::schema_identifier;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn returns_a_single_segment_unchanged() {
        assert_eq!(
            schema_identifier(&["authors"]).expect("the identifier is within the limit"),
            "authors"
        );
    }

    #[test]
    fn joins_multiple_segments_with_underscores() {
        assert_eq!(
            schema_identifier(&["articles", "created_at", "index"])
                .expect("the identifier is within the limit"),
            "articles_created_at_index"
        );
    }

    #[test]
    fn preserves_underscores_within_segments() {
        assert_eq!(
            schema_identifier(&["articles", "author_id", "index"])
                .expect("the identifier is within the limit"),
            "articles_author_id_index"
        );
    }

    #[test]
    fn is_stable_across_repeated_calls() {
        assert_eq!(
            schema_identifier(&["articles", "created_at", "index"])
                .expect("the identifier is within the limit"),
            schema_identifier(&["articles", "created_at", "index"])
                .expect("the identifier is within the limit")
        );
    }

    #[test]
    fn rejects_a_joined_identifier_over_the_byte_limit() {
        let segment = "a".repeat(MAX_IDENTIFIER_BYTES);

        let error = schema_identifier(&[segment.as_str(), "index"])
            .expect_err("the joined identifier exceeds the limit");

        let SchemaIdentifierNamingError::IdentifierTooLong { identifier, length } = error;

        assert_eq!(identifier, format!("{segment}_index"));
        assert_eq!(length, MAX_IDENTIFIER_BYTES + "_index".len());
    }
}
