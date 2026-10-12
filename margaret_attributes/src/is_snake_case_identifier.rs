use crate::is_identifier::is_identifier;
use crate::is_snake_case_name::is_snake_case_name;

#[must_use]
pub fn is_snake_case_identifier(name: &str) -> bool {
    is_snake_case_name(name) && is_identifier(name)
}

#[cfg(test)]
mod tests {
    use super::is_snake_case_identifier;

    #[test]
    fn accepts_a_snake_case_identifier_so_it_can_name_a_generated_item() {
        assert!(is_snake_case_identifier("get_article"));
    }

    #[test]
    fn rejects_an_identifier_that_contains_uppercase_so_declared_names_stay_snake_case() {
        assert!(!is_snake_case_identifier("getArticle"));
    }

    #[test]
    fn rejects_a_raw_identifier_to_keep_raw_syntax_out_of_declared_names() {
        assert!(!is_snake_case_identifier("r#type"));
    }

    #[test]
    fn rejects_a_reserved_keyword_because_generated_code_cannot_name_an_item_with_it() {
        assert!(!is_snake_case_identifier("match"));
    }
}
