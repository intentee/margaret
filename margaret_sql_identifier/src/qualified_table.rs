use crate::framework_namespace::FRAMEWORK_NAMESPACE;
use crate::quote_identifier::quote_identifier;
use crate::table_namespace::TableNamespace;

#[must_use]
pub fn qualified_table(namespace: TableNamespace, table: &str) -> String {
    match namespace {
        TableNamespace::Application => quote_identifier(table),
        TableNamespace::Framework => format!(
            "{}.{}",
            quote_identifier(FRAMEWORK_NAMESPACE),
            quote_identifier(table)
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::qualified_table;
    use crate::table_namespace::TableNamespace;

    #[test]
    fn leaves_an_application_table_in_the_search_path() {
        assert_eq!(
            qualified_table(TableNamespace::Application, "articles"),
            "\"articles\""
        );
    }

    #[test]
    fn places_a_framework_table_in_the_framework_namespace() {
        assert_eq!(
            qualified_table(TableNamespace::Framework, "signing_key_sets"),
            "\"margaret\".\"signing_key_sets\""
        );
    }
}
