#[must_use]
pub fn primary_key_index_name(table: &str) -> String {
    [table, "pkey"].join("_")
}

#[cfg(test)]
mod tests {
    use super::primary_key_index_name;

    #[test]
    fn appends_the_pkey_suffix_to_the_table() {
        assert_eq!(primary_key_index_name("authors"), "authors_pkey");
    }
}
