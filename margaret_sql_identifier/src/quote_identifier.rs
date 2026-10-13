#[must_use]
pub fn quote_identifier(identifier: &str) -> String {
    format!("\"{identifier}\"")
}

#[cfg(test)]
mod tests {
    use super::quote_identifier;

    #[test]
    fn wraps_the_identifier_in_double_quotes_so_reserved_words_like_grant_work() {
        assert_eq!(quote_identifier("grant"), "\"grant\"");
    }
}
