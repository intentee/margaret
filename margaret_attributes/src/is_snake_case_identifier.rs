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
    fn accepts_a_snake_case_identifier() {
        assert!(is_snake_case_identifier("get_article"));
    }

    #[test]
    fn rejects_an_identifier_that_contains_uppercase() {
        assert!(!is_snake_case_identifier("getArticle"));
    }

    #[test]
    fn rejects_a_raw_identifier() {
        assert!(!is_snake_case_identifier("r#type"));
    }

    #[test]
    fn rejects_a_reserved_keyword() {
        assert!(!is_snake_case_identifier("match"));
    }
}
