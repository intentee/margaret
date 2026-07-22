use margaret_schema_identifier::schema_identifier::schema_identifier;

#[must_use]
pub fn index_name(table: &str, column: &str) -> String {
    schema_identifier(&[table, column, "index"])
}

#[cfg(test)]
mod tests {
    use super::index_name;

    #[test]
    fn joins_the_table_and_column_with_an_index_suffix() {
        assert_eq!(
            index_name("articles", "created_at"),
            "articles_created_at_index"
        );
    }

    #[test]
    fn preserves_underscores_in_the_column() {
        assert_eq!(
            index_name("articles", "author_id"),
            "articles_author_id_index"
        );
    }

    #[test]
    fn is_stable_across_repeated_calls() {
        assert_eq!(
            index_name("articles", "created_at"),
            index_name("articles", "created_at")
        );
    }
}
