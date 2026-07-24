#[must_use]
pub fn unique_index_name(table: &str, columns: &[String]) -> String {
    let mut segments: Vec<&str> = Vec::with_capacity(columns.len() + 2);

    segments.push(table);
    segments.extend(columns.iter().map(String::as_str));
    segments.push("key");

    segments.join("_")
}

#[cfg(test)]
mod tests {
    use super::unique_index_name;

    #[test]
    fn appends_the_key_suffix_to_a_single_column() {
        assert_eq!(
            unique_index_name("authors", &["name".to_string()]),
            "authors_name_key"
        );
    }

    #[test]
    fn joins_every_column_before_the_key_suffix() {
        assert_eq!(
            unique_index_name("line_items", &["region".to_string(), "number".to_string()]),
            "line_items_region_number_key"
        );
    }
}
