pub(crate) fn foreign_key_column_name(field_name: &str, referenced_column_name: &str) -> String {
    format!("{field_name}_{referenced_column_name}")
}

#[cfg(test)]
mod tests {
    use crate::foreign_key_column_name::foreign_key_column_name;

    #[test]
    fn joins_the_field_name_and_referenced_column() {
        assert_eq!(foreign_key_column_name("author", "id"), "author_id");
    }

    #[test]
    fn joins_a_composite_reference_part() {
        assert_eq!(foreign_key_column_name("order", "region"), "order_region");
    }

    #[test]
    fn preserves_underscores_in_both_parts() {
        assert_eq!(
            foreign_key_column_name("primary_author", "external_id"),
            "primary_author_external_id"
        );
    }

    #[test]
    fn is_stable_across_repeated_calls() {
        assert_eq!(
            foreign_key_column_name("author", "id"),
            foreign_key_column_name("author", "id")
        );
    }
}
