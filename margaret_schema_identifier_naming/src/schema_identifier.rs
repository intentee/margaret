use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;
use crate::validate_identifier_length::validate_identifier_length;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaIdentifier {
    value: String,
}

impl SchemaIdentifier {
    pub fn from_segments(segments: &[&str]) -> Result<Self, SchemaIdentifierNamingError> {
        Self::new(segments.join("_"))
    }

    pub fn new(value: String) -> Result<Self, SchemaIdentifierNamingError> {
        validate_identifier_length(&value)?;

        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    use super::SchemaIdentifier;

    #[test]
    fn joins_segments_with_underscores() {
        assert_eq!(
            SchemaIdentifier::from_segments(&["articles", "created_at", "index"])
                .expect("the identifier is within the limit")
                .as_str(),
            "articles_created_at_index"
        );
    }

    #[test]
    fn preserves_a_single_segment_unchanged() {
        assert_eq!(
            SchemaIdentifier::new("authors".to_string())
                .expect("the identifier is within the limit")
                .as_str(),
            "authors"
        );
    }

    #[test]
    fn rejects_a_value_over_the_byte_limit() {
        let value = "a".repeat(MAX_IDENTIFIER_BYTES + 1);

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } =
            SchemaIdentifier::new(value).expect_err("an over-limit value is rejected");

        assert_eq!(length, MAX_IDENTIFIER_BYTES + 1);
    }

    #[test]
    fn rejects_joined_segments_over_the_byte_limit() {
        let segment = "a".repeat(MAX_IDENTIFIER_BYTES);

        let SchemaIdentifierNamingError::IdentifierTooLong { identifier, length } =
            SchemaIdentifier::from_segments(&[segment.as_str(), "index"])
                .expect_err("the joined identifier exceeds the limit");

        assert_eq!(identifier, format!("{segment}_index"));
        assert_eq!(length, MAX_IDENTIFIER_BYTES + "_index".len());
    }
}
