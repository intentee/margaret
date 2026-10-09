use crate::framework_namespace::FRAMEWORK_NAMESPACE;

#[must_use]
pub fn qualified_framework_table(table: &str) -> String {
    format!("\"{FRAMEWORK_NAMESPACE}\".\"{table}\"")
}

#[cfg(test)]
mod tests {
    use super::qualified_framework_table;

    #[test]
    fn quotes_the_table_inside_the_framework_namespace() {
        assert_eq!(
            qualified_framework_table("signing_key_sets"),
            "\"margaret\".\"signing_key_sets\""
        );
    }
}
