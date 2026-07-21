#[must_use]
pub fn schema_identifier(segments: &[&str]) -> String {
    segments.join("_")
}

#[cfg(test)]
mod tests {
    use super::schema_identifier;

    #[test]
    fn returns_a_single_segment_unchanged() {
        assert_eq!(schema_identifier(&["authors"]), "authors");
    }

    #[test]
    fn joins_multiple_segments_with_underscores() {
        assert_eq!(
            schema_identifier(&["articles", "created_at", "index"]),
            "articles_created_at_index"
        );
    }

    #[test]
    fn preserves_underscores_within_segments() {
        assert_eq!(
            schema_identifier(&["articles", "author_id", "index"]),
            "articles_author_id_index"
        );
    }

    #[test]
    fn is_stable_across_repeated_calls() {
        assert_eq!(
            schema_identifier(&["articles", "created_at", "index"]),
            schema_identifier(&["articles", "created_at", "index"])
        );
    }
}
