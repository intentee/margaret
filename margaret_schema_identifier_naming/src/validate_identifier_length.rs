use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

/// # Errors
///
/// Returns `SchemaIdentifierNamingError::IdentifierTooLong`.
pub fn validate_identifier_length(identifier: &str) -> Result<(), SchemaIdentifierNamingError> {
    let length = identifier.len();

    if length > MAX_IDENTIFIER_BYTES {
        return Err(SchemaIdentifierNamingError::IdentifierTooLong {
            identifier: identifier.to_string(),
            length,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_identifier_length;
    use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;
    use crate::schema_identifier_naming_error::SchemaIdentifierNamingError;

    #[test]
    fn accepts_an_identifier_at_the_byte_limit() {
        let identifier = "a".repeat(MAX_IDENTIFIER_BYTES);

        assert!(validate_identifier_length(&identifier).is_ok());
    }

    #[test]
    fn rejects_an_identifier_over_the_byte_limit() {
        let identifier = "a".repeat(MAX_IDENTIFIER_BYTES + 1);

        let error = validate_identifier_length(&identifier)
            .expect_err("an identifier over the limit is rejected");

        let SchemaIdentifierNamingError::IdentifierTooLong { length, .. } = error;

        assert_eq!(length, MAX_IDENTIFIER_BYTES + 1);
    }
}
