#[must_use]
pub fn quote_identifier(identifier: &str) -> String {
    format!("\"{identifier}\"")
}

#[cfg(test)]
mod tests {
    use super::quote_identifier;

    #[test]
    fn wraps_the_identifier_in_double_quotes() {
        assert_eq!(quote_identifier("grant"), "\"grant\"");
    }
}
